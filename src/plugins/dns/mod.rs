use std::collections::HashMap;
use std::io;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use once_cell::sync::Lazy;
use regex::Regex;
use sys_locale::get_locale;
use tokio::task::JoinSet;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{LookupIpStrategy, NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use url::Url;

#[cfg(target_os = "windows")]
use encoding_rs::GBK;

const TIMEOUT: Duration = Duration::from_secs(3);

const NAMESERVERS: &[&str] = &[
    "1.1.1.1",
    "1.2.4.8",
    "1.12.12.12",
    "4.2.2.1",
    "8.8.8.8",
    "8.20.247.20",
    "8.26.56.26",
    "9.9.9.9",
    "45.11.45.11",
    "52.80.52.52",
    "64.6.64.6",
    "74.82.42.42",
    "77.88.8.8",
    "80.80.80.80",
    "84.200.69.80",
    "94.140.14.14",
    "100.95.0.1",
    "101.101.101.101",
    "101.226.4.6",
    "114.114.114.114",
    "117.50.10.10",
    "119.29.29.29",
    "156.154.70.1",
    "168.95.1.1",
    "168.126.63.1",
    "180.76.76.76",
    "180.184.1.1",
    "182.254.118.118",
    "185.222.222.222",
    "195.46.39.39",
    "199.85.126.10",
    "202.120.2.100",
    "208.67.222.222",
    "210.2.4.8",
    "223.5.5.5",
];

const LANG_ZH: &str = "zh-CN";
const LANG_EN: &str = "en-US";

static LANG_LOCAL: Lazy<String> = Lazy::new(|| {
    let locale = get_locale().unwrap_or_else(|| String::from(LANG_EN));
    if ["zh-CN", "zh-Hans-CN"]
        .iter()
        .any(|&variant| locale.starts_with(variant))
    {
        String::from(LANG_ZH)
    } else {
        String::from(LANG_EN)
    }
});

static PING_PATTERNS: Lazy<HashMap<String, (Regex, Regex)>> = Lazy::new(|| {
    let mut patterns = HashMap::new();
    // English patterns
    patterns.insert(
        String::from(LANG_EN),
        (
            Regex::new(r"(\d+)% packet loss").unwrap(),
            Regex::new(r"Minimum = (\d+)ms, Maximum = (\d+)ms, Average = (\d+)ms").unwrap(),
        ),
    );
    // Chinese patterns
    patterns.insert(
        String::from(LANG_ZH),
        (
            Regex::new(r"丢失 = (\d+) \((\d+)% 丢失\)").unwrap(),
            Regex::new(r"最短 = (\d+)ms，最长 = (\d+)ms，平均 = (\d+)ms").unwrap(),
        ),
    );
    patterns
});

#[derive(Debug)]
struct PingMetrics {
    ip: IpAddr,
    min: u32,
    max: u32,
    avg: u32,
    loss: u32,
}

impl PingMetrics {
    fn new(ip: IpAddr) -> Self {
        Self {
            ip,
            avg: 0,
            min: 0,
            max: 0,
            loss: 0,
        }
    }
}

fn log_line(
    tx: &Sender<String>,
    line: impl Into<String>,
) {
    let _ = tx.send(line.into());
}

fn log_text(
    tx: &Sender<String>,
    text: &str,
) {
    // Preserve multi-line chunks but still stream in UI
    if text.is_empty() {
        let _ = tx.send(String::new());
        return;
    }
    for line in text.split('\n') {
        let _ = tx.send(line.to_string());
    }
}

#[cfg(target_os = "windows")]
async fn ping_task(ip: IpAddr) -> io::Result<String> {
    let timeout_ms = TIMEOUT.as_millis().max(100) as u32;
    let output = Command::new("ping")
        .args(["/n", "4", "/w", &timeout_ms.to_string(), &ip.to_string()])
        .output()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to execute ping: {}", e)))?;

    let (decoded, _, _) = GBK.decode(&output.stdout);
    Ok(decoded.into_owned())
}

#[cfg(not(target_os = "windows"))]
async fn ping_task(ip: IpAddr) -> io::Result<String> {
    let timeout_sec = TIMEOUT.as_secs().max(1);
    let output = Command::new("ping")
        .args(["-c", "4", "-W", &timeout_sec.to_string(), &ip.to_string()])
        .output()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to execute ping: {}", e)))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

async fn query_task(
    domain: &str,
    config: NameServerConfig,
    tx: &Sender<String>,
) -> io::Result<Vec<IpAddr>> {
    let mut opts = ResolverOpts::default();
    opts.timeout = TIMEOUT;
    opts.attempts = 1;
    opts.validate = false;
    opts.use_hosts_file = false;
    opts.ip_strategy = LookupIpStrategy::Ipv4Only;
    let mut resolver = ResolverConfig::new();
    resolver.add_name_server(config);

    match TokioAsyncResolver::tokio(resolver, opts).lookup_ip(domain).await {
        Ok(response) => {
            let ips: Vec<_> = response.iter().collect();
            if !ips.is_empty() {
                for ip in &ips {
                    log_line(tx, format!("Found IP for domain '{}' : {}", domain, ip));
                }
                Ok(ips)
            } else {
                Ok(vec![])
            }
        }
        Err(_) => Ok(vec![]),
    }
}

async fn query_domain(
    domain: &str,
    nameservers: &[IpAddr],
    tx: &Sender<String>,
) -> io::Result<Vec<IpAddr>> {
    let mut rets = Vec::new();
    let mut tasks = JoinSet::new();
    for &nameserver in nameservers {
        let domain = domain.to_string();
        let tx_udp = tx.clone();
        let tx_tcp = tx.clone();
        tasks.spawn(async move {
            log_line(
                &tx_udp,
                format!("Query domain '{}' using nameserver {} (UDP)", domain, nameserver),
            );
            let config = NameServerConfig {
                socket_addr: (nameserver, 53).into(),
                protocol: Protocol::Udp,
                tls_dns_name: None,
                trust_negative_responses: true,
                bind_addr: None,
            };
            let udp_result = query_task(&domain, config, &tx_udp).await;
            match udp_result {
                Ok(ips) if !ips.is_empty() => Ok(ips),
                _ => {
                    log_line(
                        &tx_tcp,
                        format!("Query domain '{}' using nameserver {} (TCP)", domain, nameserver),
                    );
                    let config = NameServerConfig {
                        socket_addr: (nameserver, 53).into(),
                        protocol: Protocol::Tcp,
                        tls_dns_name: None,
                        trust_negative_responses: true,
                        bind_addr: None,
                    };
                    query_task(&domain, config, &tx_tcp).await
                }
            }
        });
    }

    while let Some(result) = tasks.join_next().await {
        if let Ok(Ok(ips)) = result {
            for ip in ips {
                if ip == IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)) {
                    continue;
                }
                if !rets.contains(&ip) {
                    rets.push(ip);
                }
            }
        }
    }

    Ok(rets)
}

async fn run_query(
    domain: String,
    tx: Sender<String>,
) {
    // validate domain format
    let is_valid = Url::parse(&format!("https://{}", domain))
        .map(|url| url.host_str().is_some())
        .unwrap_or(false);
    if !is_valid {
        log_line(&tx, "Error: invalid domain name format");
        return;
    }

    // nameservers
    let nameservers: Vec<IpAddr> = NAMESERVERS.iter().filter_map(|&s| s.parse().ok()).collect();
    if nameservers.is_empty() {
        log_line(&tx, "Error: no valid nameservers found");
        return;
    }

    // query domain
    let all_ips = match query_domain(&domain, &nameservers, &tx).await {
        Ok(v) => v,
        Err(e) => {
            log_line(&tx, format!("Query failed: {}", e));
            return;
        }
    };
    log_line(&tx, String::new());
    log_line(&tx, "Query domain successful");

    if !all_ips.is_empty() {
        log_line(&tx, String::new());
        log_line(&tx, "Check results");
        let mut tasks = JoinSet::new();
        for ip in all_ips {
            tasks.spawn(async move { ping_task(ip).await.map(|output| (ip, output)) });
        }

        let mut results: Vec<PingMetrics> = Vec::new();
        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(Ok((ip, output))) => {
                    log_text(&tx, &output);

                    // parse metrics
                    let mut metrics = PingMetrics::new(ip);
                    let language = LANG_LOCAL.clone();
                    let patterns = PING_PATTERNS
                        .get(&language)
                        .unwrap_or_else(|| panic!("Error: Unsupported language: {}", language));

                    if let Some(caps) = patterns.0.captures(&output) {
                        metrics.loss = if language == LANG_ZH {
                            caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(100)
                        } else {
                            caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(100)
                        };
                    }
                    if let Some(caps) = patterns.1.captures(&output) {
                        metrics.min = caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
                        metrics.max = caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
                        metrics.avg = caps.get(3).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
                    }
                    results.push(metrics);
                }
                Ok(Err(err)) => log_line(&tx, format!("Run ping command failed: {}", err)),
                Err(err) => log_line(&tx, format!("Run ping command failed: {}", err)),
            }
        }

        if !results.is_empty() {
            log_line(&tx, "Display results（sorted）");
            results.sort_by(|a, b| {
                a.loss
                    .cmp(&b.loss)
                    .then(a.avg.cmp(&b.avg))
                    .then(a.max.cmp(&b.max))
                    .then(a.min.cmp(&b.min))
            });

            log_line(&tx, String::new());
            for metrics in results {
                log_line(
                    &tx,
                    format!(
                        "IP: {}, Avg Latency: {}ms, Min Latency: {}ms, Max Latency: {}ms, Packet Loss: {}%",
                        metrics.ip, metrics.avg, metrics.min, metrics.max, metrics.loss
                    ),
                );
            }
        }
    } else {
        log_line(&tx, String::new());
        log_line(&tx, "no ip addresses found");
    }
}

/// Start a DNS query for a domain and return a receiver of log lines.
/// The query runs in a background thread with its own Tokio runtime.
pub fn start_dns_query(domain: String) -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    std::thread::spawn(move || {
        let runtime = match tokio::runtime::Builder::new_multi_thread().enable_all().build() {
            Ok(rt) => rt,
            Err(e) => {
                log_line(&tx, format!("Failed to create runtime: {}", e));
                return;
            }
        };
        runtime.block_on(run_query(domain, tx));
    });

    rx
}
