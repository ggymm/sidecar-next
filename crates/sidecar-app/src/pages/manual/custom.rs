use std::{fs, path::PathBuf};

use gpui::*;
use gpui_component::{Disableable, Placement, StyledExt, WindowExt as _, input::InputState, text::TextView};
use serde::Deserialize;

use crate::{
    MainView,
    comps::{button, card, page, textarea},
};

const ROOT_PATH: &str = "assets/manual/custom/";

#[derive(Clone, Debug, Deserialize)]
struct ManualEntry {
    name: String,
    #[serde(default)]
    keywords: Vec<String>,
}

pub struct CustomPage {
    search: Entity<InputState>,
    entries: Vec<ManualEntry>,
    display_entries: Vec<ManualEntry>,
    loading: bool,
}

impl CustomPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let index = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(ROOT_PATH)
                .join("index.json");

            let search = cx.new(|cx| InputState::new(window, cx));
            let entries = fs::read_to_string(&index)
                .and_then(|data| serde_json::from_str::<Vec<ManualEntry>>(&data).map_err(Into::into))
                .unwrap_or_default();
            let display_entries = entries.clone();
            Self {
                search,
                entries,
                display_entries,
                loading: false,
            }
        }))
    }

    fn search(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.loading {
            return;
        }
        let search = self.search.read(cx).value().trim().to_string();
        let entries = self.entries.clone();

        self.loading = true;
        cx.notify();

        cx.spawn_in(window, async move |page, cx| {
            let display_entries = cx
                .background_executor()
                .spawn({
                    let query = search.clone().to_lowercase();
                    if query.is_empty() {
                        return;
                    }
                    async move {
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
                    page.loading = false;
                    page.display_entries = display_entries;
                    cx.notify();
                });
            });
        })
        .detach();
    }

    fn display(
        &self,
        entry: ManualEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name = entry.name.clone();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(ROOT_PATH)
            .join(format!("{}.md", entry.name));

        let markdown = fs::read_to_string(&path).unwrap_or_default();
        window.open_sheet_at(Placement::Right, cx, move |drawer, window, app| {
            let content = TextView::markdown("custom-manual", markdown.clone(), window, app).selectable(true);

            drawer
                .title(
                    div()
                        .px_4()
                        .py_2()
                        .text_lg()
                        .font_semibold()
                        .text_color(white())
                        .child(name.clone()),
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

impl Render for CustomPage {
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
                        .flex_row()
                        .gap_4()
                        .items_center()
                        .child(textarea(&self.search, |input| input))
                        .child(
                            button(cx, "manual-refresh")
                                .label(if self.loading { "查询中..." } else { "查询" })
                                .disabled(self.loading)
                                .on_click(cx.listener(|this, _ev, window, cx| {
                                    this.search(window, cx);
                                })),
                        ),
                ),
            )
            .child(card().flex_1().child(self.display_entries.iter().cloned().fold(
                div().flex().flex_col().flex_1().pr_4().gap_2(),
                |acc, entry| {
                    let entry_for_click = entry.clone();
                    acc.child(
                        div()
                            .p_2()
                            .hover(|style| style.p_4().bg(rgb(0x383838)))
                            .rounded_lg()
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _ev, window, cx| {
                                    this.display(entry_for_click.clone(), window, cx);
                                }),
                            )
                            .child(div().text_base().text_color(white()).child(entry.name.clone())),
                    )
                },
            )))
            .into_any_element()
    }
}
