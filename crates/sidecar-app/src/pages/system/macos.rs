use std::process::Command;

use gpui::*;
use gpui_component::{WindowExt, input::InputState};

use crate::{
    MainView,
    comps::{button, card, label, page, textarea},
};

fn read_dock_size() -> Result<i32, String> {
    let output = Command::new("defaults")
        .args(["read", "com.apple.dock", "tilesize"])
        .output()
        .map_err(|e| format!("执行命令失败: {}", e))?;

    if !output.status.success() {
        return Err("读取 Dock 大小失败".to_string());
    }

    let size_str = String::from_utf8_lossy(&output.stdout);
    size_str
        .trim()
        .parse::<i32>()
        .map_err(|e| format!("解析数值失败: {}", e))
}

fn write_dock_size(size: i32) -> Result<(), String> {
    if !(16..=128).contains(&size) {
        return Err("大小必须在 16-128 之间".to_string());
    }

    // 写入配置
    let write_output = Command::new("defaults")
        .args(["write", "com.apple.dock", "tilesize", "-int", &size.to_string()])
        .output()
        .map_err(|e| format!("执行命令失败: {}", e))?;

    if !write_output.status.success() {
        return Err("设置 Dock 大小失败".to_string());
    }

    Ok(())
}

pub struct MacosPage {
    dock_size: Entity<InputState>,
}

impl MacosPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let dock_size = cx.new(|cx| InputState::new(window, cx).placeholder("请输入 16-128 之间的数字"));

            Self { dock_size }
        }))
    }
}

impl Render for MacosPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        page()
            .size_full()
            .child(
                card().child(label("程序坞（Dock）高度设置")).child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_4()
                        .items_center()
                        .child(textarea(&self.dock_size, |input| input))
                        .child(button(cx, "read-dock-size").label("读取").on_click(cx.listener({
                            let size_entity = self.dock_size.clone();
                            move |_, _, window, cx| match read_dock_size() {
                                Ok(size) => {
                                    size_entity.update(cx, |state, cx2| {
                                        state.set_value(size.to_string(), window, cx2);
                                    });
                                    window.push_notification("已读取当前大小", cx);
                                }
                                Err(e) => {
                                    window.push_notification(format!("读取失败: {}", e), cx);
                                }
                            }
                        })))
                        .child(button(cx, "write-dock-size").label("应用").on_click(cx.listener({
                            let size_entity = self.dock_size.clone();
                            move |_, _, window, cx| match size_entity.read(cx).value().parse::<i32>() {
                                Ok(size) => match write_dock_size(size) {
                                    Ok(_) => {
                                        window.push_notification("已应用当前大小", cx);
                                    }
                                    Err(e) => {
                                        window.push_notification(format!("应用失败: {}", e), cx);
                                    }
                                },
                                Err(_) => {
                                    window.push_notification("请输入有效的数字", cx);
                                }
                            }
                        }))),
                ),
            )
            .child(div())
    }
}
