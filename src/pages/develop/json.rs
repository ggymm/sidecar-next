use std::collections::HashSet;

use gpui::prelude::*;
use gpui::*;
use gpui::{InteractiveElement, StatefulInteractiveElement};
use gpui_component::input::InputEvent;
use gpui_component::input::InputState;
use serde_json::Value;

use crate::MainView;
use crate::comps::button;
use crate::comps::card;
use crate::comps::input;
use crate::comps::label;
use crate::comps::page;
use crate::comps::textarea;

const INDENT_UNIT: &str = "\t";

pub struct JsonPage {
    input: Entity<InputState>,
    search: Entity<InputState>,
    lines: Vec<JsonLine>,
    collapsed: HashSet<JsonPath>,
    search_query: Option<String>,
    parse_error: Option<String>,
    _subs: Vec<Subscription>,
}

impl JsonPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input = cx.new(|cx| {
                InputState::new(window, cx)
                    .multi_line()
                    .placeholder("粘贴或输入 JSON 文本，点击右上角“格式化”")
            });
            let search = cx.new(|cx| InputState::new(window, cx).placeholder("输入关键字，实时匹配"));

            let subs = vec![cx.subscribe_in(&search, window, Self::on_search_event)];

            Self {
                input,
                search,
                lines: Vec::new(),
                collapsed: HashSet::new(),
                search_query: None,
                parse_error: None,
                _subs: subs,
            }
        }))
    }

    fn format_json(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let raw = self.input.read(cx).value().to_string();
        if raw.trim().is_empty() {
            self.lines.clear();
            self.parse_error = None;
            self.collapsed.clear();
            cx.notify();
            return;
        }

        match serde_json::from_str::<Value>(&raw) {
            Ok(value) => {
                let pretty = serde_json::to_string_pretty(&value).unwrap_or_else(|_| raw.clone());
                self.input.update(cx, |state, cx2| {
                    state.set_value(pretty.clone(), window, cx2);
                });

                let lines = JsonLineBuilder::build(&value);
                let available_paths: HashSet<JsonPath> = lines
                    .iter()
                    .filter_map(|line| if line.is_collapsible() { line.path.clone() } else { None })
                    .collect();

                self.collapsed.retain(|path| available_paths.contains(path));
                self.lines = lines;
                self.parse_error = None;
            }
            Err(err) => {
                self.lines.clear();
                self.parse_error = Some(err.to_string());
            }
        }
        cx.notify();
    }

    fn toggle_collapse(
        &mut self,
        line_idx: usize,
        cx: &mut Context<Self>,
    ) {
        if let Some(line) = self.lines.get(line_idx) {
            if !line.is_collapsible() {
                return;
            }
            if let Some(path) = line.path.clone() {
                if !self.collapsed.remove(&path) {
                    self.collapsed.insert(path);
                }
                cx.notify();
            }
        }
    }

    fn collapse_all(
        &mut self,
        cx: &mut Context<Self>,
    ) {
        let mut collapsed = HashSet::new();
        for line in &self.lines {
            if line.is_collapsible() {
                if let Some(path) = line.path.clone() {
                    collapsed.insert(path);
                }
            }
        }
        self.collapsed = collapsed;
        cx.notify();
    }

    fn expand_all(
        &mut self,
        cx: &mut Context<Self>,
    ) {
        if self.collapsed.is_empty() {
            return;
        }
        self.collapsed.clear();
        cx.notify();
    }

    fn set_search_query(
        &mut self,
        query: &str,
    ) {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            self.search_query = None;
        } else {
            self.search_query = Some(trimmed.to_string());
        }
    }

    fn on_search_event(
        &mut self,
        state: &Entity<InputState>,
        ev: &InputEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match ev {
            InputEvent::Change | InputEvent::PressEnter { .. } => {
                let value = state.read(cx).value().to_string();
                self.set_search_query(&value);
                cx.notify();
            }
            _ => {}
        }
    }

    fn visible_lines(&self) -> Vec<VisibleLine<'_>> {
        let mut visible = Vec::new();
        let mut hidden_ranges: Vec<(usize, usize)> = Vec::new();
        let mut number = 1usize;

        for (idx, line) in self.lines.iter().enumerate() {
            hidden_ranges.retain(|(_, end)| *end >= idx);
            let hidden = hidden_ranges.iter().any(|(start, end)| idx > *start && idx <= *end);
            if hidden {
                continue;
            }

            let collapsed = line
                .path
                .as_ref()
                .map(|path| self.collapsed.contains(path))
                .unwrap_or(false);

            if collapsed {
                if let Some(close) = line.pair_index {
                    hidden_ranges.push((idx, close));
                }
            }

            visible.push(VisibleLine {
                index: idx,
                number,
                collapsed,
                line,
            });
            number += 1;
        }

        visible
    }

    fn render_segment(segment: &JsonSegment) -> Div {
        let display = if segment.kind == JsonTokenKind::Whitespace {
            segment.text.replace('\t', "    ")
        } else {
            segment.text.clone()
        };

        let mut piece = div().child(display);
        match segment.kind {
            JsonTokenKind::Key => piece = piece.text_color(rgb(0x9cdcfe)),
            JsonTokenKind::String => piece = piece.text_color(rgb(0xce9178)),
            JsonTokenKind::Number => piece = piece.text_color(rgb(0xb5cea8)),
            JsonTokenKind::Boolean => piece = piece.text_color(rgb(0x569cd6)),
            JsonTokenKind::Null => piece = piece.text_color(rgb(0x569cd6)),
            JsonTokenKind::Punctuation => piece = piece.text_color(rgb(0xd4d4d4)),
            JsonTokenKind::Whitespace => {}
        }
        piece
    }

    fn render_line(
        &self,
        cx: &mut Context<Self>,
        entry: &VisibleLine<'_>,
        query: Option<&str>,
    ) -> Div {
        let highlight = query
            .map(|needle| entry.line.text.to_lowercase().contains(needle))
            .unwrap_or(false);

        let mut row = div()
            .flex()
            .items_start()
            .gap_3()
            .px_3()
            .py_1()
            .font_family("monospace");
        if highlight {
            row = row.bg(rgb(0x2d2f33));
        }

        row = row.child(
            div()
                .w(px(48.0))
                .flex()
                .justify_end()
                .text_color(rgb(0x6b6b6b))
                .child(format!("{:>4}", entry.number)),
        );

        let mut content = div().flex().items_start().gap_2().flex_1();

        if entry.line.is_collapsible() {
            let symbol = if entry.collapsed { "+" } else { "-" };
            let line_idx = entry.index;
            content = content.child(
                div()
                    .id(SharedString::from(format!("json-toggle-{}", line_idx)))
                    .w(px(16.0))
                    .flex()
                    .justify_center()
                    .items_center()
                    .text_color(rgb(0xb0b0b0))
                    .child(symbol)
                    .on_click(cx.listener(move |this, _ev: &ClickEvent, _window, cx| {
                        this.toggle_collapse(line_idx, cx);
                    })),
            );
        } else {
            content = content.child(div().w(px(16.0)));
        }

        let mut segments = div().flex().items_start();
        for segment in &entry.line.segments {
            segments = segments.child(Self::render_segment(segment));
        }
        if entry.collapsed {
            segments = segments.child(div().text_color(rgb(0x6b6b6b)).child(" ..."));
        }

        row.child(content.child(segments))
    }
}

impl Render for JsonPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let search_value = self.search_query.as_ref().map(|s| s.to_lowercase()).unwrap_or_default();
        let query = if search_value.is_empty() {
            None
        } else {
            Some(search_value.as_str())
        };

        let total_matches = query
            .map(|needle| {
                self.lines
                    .iter()
                    .filter(|line| line.text.to_lowercase().contains(needle))
                    .count()
            })
            .unwrap_or(0);
        let visible = self.visible_lines();

        let mut viewer = card().flex_1().min_h_0().child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(label("JSON 查看器"))
                .child(
                    div()
                        .flex()
                        .gap_3()
                        .items_center()
                        .child(input(&self.search).cleanable().font_family("monospace"))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(0x8a8a8a))
                                .child(format!("匹配行：{}", total_matches)),
                        )
                        .child(button(cx, "json-collapse-all").label("全部折叠").on_click(cx.listener(
                            |this, _ev, _window, cx| {
                                this.collapse_all(cx);
                            },
                        )))
                        .child(button(cx, "json-expand-all").label("全部展开").on_click(cx.listener(
                            |this, _ev, _window, cx| {
                                this.expand_all(cx);
                            },
                        ))),
                ),
        );

        if let Some(err) = self.parse_error.as_ref() {
            viewer = viewer.child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x3a1f1f))
                    .text_color(rgb(0xff9393))
                    .text_sm()
                    .child(format!("解析失败：{}", err)),
            );
        }

        let mut lines_container = div().flex().flex_col().font_family("monospace").text_sm();

        for entry in &visible {
            lines_container = lines_container.child(self.render_line(cx, entry, query).into_any_element());
        }

        if visible.is_empty() {
            lines_container = div()
                .flex()
                .items_center()
                .justify_center()
                .h_full()
                .py_20()
                .text_color(rgb(0x707070))
                .text_sm()
                .child("格式化有效的 JSON 后将在此处显示结果");
        }

        viewer = viewer.child(
            div()
                .id("json-viewer-scroll")
                .flex_1()
                .min_h_0()
                .bg(rgb(0x1f1f1f))
                .border_1()
                .border_color(rgb(0x353535))
                .rounded_lg()
                .overflow_y_scroll()
                .child(lines_container),
        );

        let input_panel =
            card()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(label("JSON 输入"))
                        .child(button(cx, "json-format").label("格式化").on_click(cx.listener(
                            |this, _ev, window, cx| {
                                this.format_json(window, cx);
                            },
                        ))),
                )
                .child(textarea(&self.input, |input| {
                    input.font_family("monospace").cleanable()
                }));

        page().size_full().child(
            div()
                .flex()
                .flex_col()
                .gap_5()
                .flex_1()
                .min_h_0()
                .child(input_panel)
                .child(viewer),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct JsonPath {
    segments: Vec<JsonPathSegment>,
}

impl JsonPath {
    fn root() -> Self {
        Self { segments: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum JsonPathSegment {
    Key(String),
    Index(usize),
}

#[derive(Clone)]
struct JsonLine {
    segments: Vec<JsonSegment>,
    kind: JsonLineKind,
    pair_index: Option<usize>,
    path: Option<JsonPath>,
    text: String,
}

impl JsonLine {
    fn is_collapsible(&self) -> bool {
        matches!(self.kind, JsonLineKind::ContainerStart)
    }
}

#[derive(Clone)]
struct JsonSegment {
    text: String,
    kind: JsonTokenKind,
}

impl JsonSegment {
    fn new(
        kind: JsonTokenKind,
        text: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            text: text.into(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum JsonTokenKind {
    Whitespace,
    Key,
    String,
    Number,
    Boolean,
    Null,
    Punctuation,
}

#[derive(Clone, Copy)]
enum JsonLineKind {
    ContainerStart,
    ContainerEnd,
    Leaf,
}

struct VisibleLine<'a> {
    index: usize,
    number: usize,
    collapsed: bool,
    line: &'a JsonLine,
}

struct JsonLineBuilder {
    lines: Vec<JsonLine>,
}

impl JsonLineBuilder {
    fn build(value: &Value) -> Vec<JsonLine> {
        let mut builder = Self { lines: Vec::new() };
        match value {
            Value::Object(map) => builder.emit_object(None, map, 0, JsonPath::root(), false),
            Value::Array(items) => builder.emit_array(None, items, 0, JsonPath::root(), false),
            other => builder.emit_scalar(None, Self::scalar_segment(other), 0, false),
        }
        builder.lines
    }

    fn emit_value(
        &mut self,
        key: Option<&str>,
        value: &Value,
        indent: usize,
        path: JsonPath,
        trailing_comma: bool,
    ) {
        match value {
            Value::Object(map) => self.emit_object(key, map, indent, path, trailing_comma),
            Value::Array(items) => self.emit_array(key, items, indent, path, trailing_comma),
            other => {
                let segment = Self::scalar_segment(other);
                self.emit_scalar(key, segment, indent, trailing_comma);
            }
        }
    }

    fn emit_object(
        &mut self,
        key: Option<&str>,
        map: &serde_json::Map<String, Value>,
        indent: usize,
        path: JsonPath,
        trailing_comma: bool,
    ) {
        if map.is_empty() {
            let mut segments = vec![indent_segment(indent)];
            if let Some(key) = key {
                segments.extend(key_segments(key));
            }
            segments.push(JsonSegment::new(JsonTokenKind::Punctuation, "{}"));
            if trailing_comma {
                segments.push(JsonSegment::new(JsonTokenKind::Punctuation, ","));
            }
            self.push_line(segments, JsonLineKind::Leaf, None);
            return;
        }

        let mut segments = vec![indent_segment(indent)];
        if let Some(key) = key {
            segments.extend(key_segments(key));
        }
        segments.push(JsonSegment::new(JsonTokenKind::Punctuation, "{"));

        let open_index = self.push_line(segments, JsonLineKind::ContainerStart, Some(path.clone()));

        let len = map.len();
        for (idx, (child_key, child_val)) in map.iter().enumerate() {
            let mut child_path = path.clone();
            child_path.segments.push(JsonPathSegment::Key(child_key.clone()));
            self.emit_value(
                Some(child_key.as_str()),
                child_val,
                indent + 1,
                child_path,
                idx + 1 < len,
            );
        }

        let mut closing = vec![indent_segment(indent)];
        closing.push(JsonSegment::new(JsonTokenKind::Punctuation, "}"));
        if trailing_comma {
            closing.push(JsonSegment::new(JsonTokenKind::Punctuation, ","));
        }
        let close_index = self.push_line(closing, JsonLineKind::ContainerEnd, None);
        if let Some(line) = self.lines.get_mut(open_index) {
            line.pair_index = Some(close_index);
        }
    }

    fn emit_array(
        &mut self,
        key: Option<&str>,
        items: &[Value],
        indent: usize,
        path: JsonPath,
        trailing_comma: bool,
    ) {
        if items.is_empty() {
            let mut segments = vec![indent_segment(indent)];
            if let Some(key) = key {
                segments.extend(key_segments(key));
            }
            segments.push(JsonSegment::new(JsonTokenKind::Punctuation, "[]"));
            if trailing_comma {
                segments.push(JsonSegment::new(JsonTokenKind::Punctuation, ","));
            }
            self.push_line(segments, JsonLineKind::Leaf, None);
            return;
        }

        let mut segments = vec![indent_segment(indent)];
        if let Some(key) = key {
            segments.extend(key_segments(key));
        }
        segments.push(JsonSegment::new(JsonTokenKind::Punctuation, "["));

        let open_index = self.push_line(segments, JsonLineKind::ContainerStart, Some(path.clone()));

        for (idx, item) in items.iter().enumerate() {
            let mut child_path = path.clone();
            child_path.segments.push(JsonPathSegment::Index(idx));
            self.emit_value(None, item, indent + 1, child_path, idx + 1 < items.len());
        }

        let mut closing = vec![indent_segment(indent)];
        closing.push(JsonSegment::new(JsonTokenKind::Punctuation, "]"));
        if trailing_comma {
            closing.push(JsonSegment::new(JsonTokenKind::Punctuation, ","));
        }
        let close_index = self.push_line(closing, JsonLineKind::ContainerEnd, None);
        if let Some(line) = self.lines.get_mut(open_index) {
            line.pair_index = Some(close_index);
        }
    }

    fn emit_scalar(
        &mut self,
        key: Option<&str>,
        content: JsonSegment,
        indent: usize,
        trailing_comma: bool,
    ) {
        let mut segments = vec![indent_segment(indent)];
        if let Some(key) = key {
            segments.extend(key_segments(key));
        }
        segments.push(content);
        if trailing_comma {
            segments.push(JsonSegment::new(JsonTokenKind::Punctuation, ","));
        }
        self.push_line(segments, JsonLineKind::Leaf, None);
    }

    fn push_line(
        &mut self,
        segments: Vec<JsonSegment>,
        kind: JsonLineKind,
        path: Option<JsonPath>,
    ) -> usize {
        let text = segments.iter().map(|seg| seg.text.as_str()).collect::<String>();
        self.lines.push(JsonLine {
            segments,
            kind,
            pair_index: None,
            path,
            text,
        });
        self.lines.len() - 1
    }

    fn scalar_segment(value: &Value) -> JsonSegment {
        match value {
            Value::String(s) => {
                let text = serde_json::to_string(s).unwrap_or_else(|_| format!("\"{}\"", s));
                JsonSegment::new(JsonTokenKind::String, text)
            }
            Value::Number(n) => JsonSegment::new(JsonTokenKind::Number, n.to_string()),
            Value::Bool(b) => JsonSegment::new(JsonTokenKind::Boolean, b.to_string()),
            Value::Null => JsonSegment::new(JsonTokenKind::Null, "null"),
            _ => JsonSegment::new(JsonTokenKind::Punctuation, "null"),
        }
    }
}

fn indent_segment(indent: usize) -> JsonSegment {
    JsonSegment::new(JsonTokenKind::Whitespace, INDENT_UNIT.repeat(indent))
}

fn key_segments(key: &str) -> Vec<JsonSegment> {
    let quoted = serde_json::to_string(key).unwrap_or_else(|_| format!("\"{}\"", key));
    vec![
        JsonSegment::new(JsonTokenKind::Key, quoted),
        JsonSegment::new(JsonTokenKind::Punctuation, ": "),
    ]
}
