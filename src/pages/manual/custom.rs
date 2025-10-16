use std::fs;
use std::path::PathBuf;

use gpui::MouseButton;
use gpui::*;
use gpui_component::ContextModal;
use gpui_component::Disableable;
use gpui_component::StyledExt;
use gpui_component::input::InputState;
use gpui_component::text::TextView;
use serde::Deserialize;

use crate::MainView;
use crate::comps::button;
use crate::comps::card;
use crate::comps::page;
use crate::comps::textarea;

const MANUAL_INDEX_PATH: &str = "assets/manual/custom/index.json";
const MANUAL_DIR: &str = "assets/manual/custom";

#[derive(Clone, Debug, Deserialize)]
struct ManualEntry {
    name: String,
    #[serde(default)]
    keywords: Vec<String>,
}

pub struct CustomManualPage {
    search: Entity<InputState>,
    entries: Vec<ManualEntry>,
    display_entries: Vec<ManualEntry>,
    filter_query: String,
    searching: bool,
}

impl CustomManualPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let search = cx.new(|cx| InputState::new(window, cx));

            let index_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(MANUAL_INDEX_PATH);
            let (entries, display_entries, _) = match fs::read_to_string(&index_path) {
                Ok(data) => match serde_json::from_str::<Vec<ManualEntry>>(&data) {
                    Ok(entries) => {
                        let display = entries.clone();
                        (entries, display, None)
                    }
                    Err(err) => (
                        Vec::new(),
                        Vec::new(),
                        Some(format!("解析索引失败：{}\n路径：{}", err, index_path.display())),
                    ),
                },
                Err(err) => (
                    Vec::new(),
                    Vec::new(),
                    Some(format!("读取索引失败：{}\n路径：{}", err, index_path.display())),
                ),
            };

            Self {
                search,
                entries,
                filter_query: String::new(),
                display_entries,
                searching: false,
            }
        }))
    }

    fn start_search(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.searching {
            return;
        }

        let filter_query = self.search.read(cx).value().trim().to_string();
        let entries = self.entries.clone();

        self.searching = true;
        cx.notify();

        cx.spawn_in(window, async move |page, cx| {
            let filtered = cx
                .background_executor()
                .spawn({
                    let query = filter_query.clone();
                    async move {
                        if query.is_empty() {
                            return entries;
                        }
                        let query = query.to_lowercase();
                        entries
                            .into_iter()
                            .filter(|entry| {
                                entry.name.to_lowercase().contains(&query)
                                    || entry.keywords.iter().any(|kw| kw.to_lowercase().contains(&query))
                            })
                            .collect()
                    }
                })
                .await;

            let _ = cx.update(|_window, cx| {
                let _ = page.update(cx, |page, cx| {
                    page.filter_query = filter_query.clone();
                    page.display_entries = filtered;
                    page.searching = false;
                    cx.notify();
                });
            });
        })
        .detach();
    }

    fn open_entry(
        &self,
        entry: ManualEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let title = entry.name.clone();
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(MANUAL_DIR)
            .join(format!("{}.md", entry.name));

        let markdown = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(err) => format!(
                "# 文档加载失败\n\n路径：`{}`\n\n错误信息：```\n{}\n```",
                file_path.display(),
                err
            ),
        };

        window.open_drawer(cx, move |drawer, window, app| {
            let content = TextView::markdown("custom-manual", markdown.clone(), window, app).selectable();

            drawer
                .title(
                    div()
                        .px_4()
                        .py_3()
                        .text_lg()
                        .font_semibold()
                        .text_color(white())
                        .child(title.clone()),
                )
                .size(px(640.0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .w_full()
                        .child(div().flex_1().min_h_0().child(content)),
                )
        });
    }
}

impl Render for CustomManualPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        page()
            .size_full()
            .child(
                card().child(
                    div()
                        .flex()
                        .items_center()
                        .gap_5()
                        .child(textarea(&self.search, |input| input))
                        .child(
                            button(cx, "manual-refresh")
                                .label(if self.searching { "查询中..." } else { "查询" })
                                .disabled(self.searching)
                                .on_click(cx.listener(|this, _ev, window, cx| {
                                    this.start_search(window, cx);
                                })),
                        ),
                ),
            )
            .child(
                card()
                    .flex_1()
                    .min_h_0()
                    .child(
                        self.display_entries
                            .iter()
                            .cloned()
                            .fold(div().flex().flex_col().gap_3(), |acc, entry| {
                                let entry_for_click = entry.clone();
                                acc.child(
                                    card()
                                        .hover(|style| style.bg(rgb(0x383838)))
                                        .cursor_pointer()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _ev, window, cx| {
                                                this.open_entry(entry_for_click.clone(), window, cx);
                                            }),
                                        )
                                        .child(
                                            div().flex().items_center().justify_between().gap_4().child(
                                                div()
                                                    .text_base()
                                                    .font_normal()
                                                    .text_color(white())
                                                    .child(entry.name.clone()),
                                            ),
                                        ),
                                )
                            }),
                    ),
            )
            .into_any_element()
    }
}
