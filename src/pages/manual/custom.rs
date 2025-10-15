use std::fs;
use std::path::PathBuf;

use gpui::MouseButton;
use gpui::*;
use gpui_component::ContextModal;
use gpui_component::StyledExt;
use gpui_component::input::InputState;
use gpui_component::text::TextView;

use serde::Deserialize;

use crate::MainView;
use crate::comps::button;
use crate::comps::card;
use crate::comps::label;
use crate::comps::page;
use crate::comps::textarea;

const MANUAL_INDEX_PATH: &str = "assets/manual/custom/index.json";

#[derive(Clone, Debug, Deserialize)]
struct ManualEntry {
    name: String,
    #[serde(default)]
    keywords: Vec<String>,
}

pub struct CustomManualPage {
    search: Entity<InputState>,
    entries: Vec<ManualEntry>,
    load_error: Option<String>,
}

impl CustomManualPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let search = cx.new(|cx| InputState::new(window, cx).placeholder("搜索文档关键词"));
            {
                let search_clone = search.clone();
                let _ = cx.observe(&search_clone, |_this, _input, cx| {
                    cx.notify();
                });
            }

            let (entries, load_error) = match Self::load_entries() {
                Ok(entries) => (entries, None),
                Err(err) => (Vec::new(), Some(err)),
            };

            Self {
                search,
                entries,
                load_error,
            }
        }))
    }

    fn load_entries() -> Result<Vec<ManualEntry>, String> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let index_path = manifest_dir.join(MANUAL_INDEX_PATH);

        let data = fs::read_to_string(&index_path)
            .map_err(|err| format!("读取索引失败：{}\n路径：{}", err, index_path.display()))?;

        serde_json::from_str::<Vec<ManualEntry>>(&data)
            .map_err(|err| format!("解析索引失败：{}\n路径：{}", err, index_path.display()))
    }

    fn manual_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/manual/custom")
    }

    fn filtered_entries(
        &self,
        query: &str,
    ) -> Vec<&ManualEntry> {
        if query.trim().is_empty() {
            return self.entries.iter().collect();
        }

        let query = query.to_lowercase();
        self.entries
            .iter()
            .filter(|entry| {
                if entry.name.to_lowercase().contains(&query) {
                    return true;
                }
                entry.keywords.iter().any(|kw| kw.to_lowercase().contains(&query))
            })
            .collect()
    }

    fn open_entry(
        &self,
        entry: ManualEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let title = entry.name.clone();
        let file_path = Self::manual_root().join(format!("{}.md", entry.name));

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
        if let Some(err) = &self.load_error {
            return page()
                .child(
                    card()
                        .child(label("手册加载失败"))
                        .child(div().text_sm().text_color(rgb(0xff7777)).child(err.clone())),
                )
                .into_any_element();
        }

        let query = self.search.read(cx).value().to_string();
        let results = self.filtered_entries(&query);
        let has_query = !query.trim().is_empty();
        let is_empty = results.is_empty();

        let mut list = div().flex().flex_col().gap_3();
        for entry in results.into_iter() {
            let entry_clone = entry.clone();
            let entry_for_click = entry_clone.clone();

            list = list.child(
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
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_4()
                            .child(
                                div()
                                    .text_base()
                                    .font_semibold()
                                    .text_color(white())
                                    .child(entry_clone.name.clone()),
                            )
                            .child(div().text_sm().text_color(rgb(0x8f8f8f)).child("打开")),
                    ),
            );
        }

        if is_empty {
            list = list.child(
                div()
                    .py_6()
                    .text_center()
                    .text_sm()
                    .text_color(rgb(0x909090))
                    .child(if has_query {
                        "没有匹配的文档"
                    } else {
                        "暂未收录文档"
                    }),
            );
        }

        page()
            .child(
                card().child(
                    div()
                        .flex()
                        .items_center()
                        .gap_5()
                        .child(textarea(&self.search, |input| input))
                        .child(button(cx, "manual-refresh").label("查询").on_click(cx.listener(
                            |_this, _ev, _window, cx| {
                                cx.notify();
                            },
                        ))),
                ),
            )
            .child(
                card()
                    .flex_1()
                    .min_h_0()
                    .child(div().flex().flex_col().gap_3().child(list)),
            )
            .into_any_element()
    }
}
