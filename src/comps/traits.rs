/// Trait to mark components that can be toggled between enabled and disabled states.
use gpui::{App, FocusHandle, Window};

pub trait Disableable: Sized {
    /// Return a copy of the component with the disabled flag set.
    fn disabled(self, disabled: bool) -> Self;
}

/// Trait for view types that support a collapsed/expanded state.
pub trait Collapsible: Sized {
    fn collapsed(self, collapsed: bool) -> Self;
    fn is_collapsed(&self) -> bool;
}

/// Trait providing focus cycling helper behaviour.
pub trait FocusableCycle {
    fn cycle_focus_handles(&self, window: &mut Window, cx: &mut App) -> Vec<FocusHandle>
    where
        Self: Sized;

    fn cycle_focus(&self, is_next: bool, window: &mut Window, cx: &mut App)
    where
        Self: Sized,
    {
        let focused_handle = window.focused(cx);
        let mut handles = self.cycle_focus_handles(window, cx);
        if handles.is_empty() {
            return;
        }
        if !is_next {
            handles.reverse();
        }
        let fallback = handles[0].clone();
        let target = handles
            .into_iter()
            .skip_while(|handle| Some(handle) != focused_handle.as_ref())
            .skip(1)
            .next()
            .unwrap_or(fallback);
        target.focus(window);
        cx.stop_propagation();
    }
}
