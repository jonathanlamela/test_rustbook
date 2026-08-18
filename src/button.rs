use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme as _;
use gpui_preview::Previewable;

#[derive(Clone, Default, Previewable)]
#[preview(category = "Inputs")]
pub struct Button {
    /// The text displayed on the button.
    pub label: String,
    /// Whether the button is interactive.
    pub disabled: bool,
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id("preview-button")
            .px_4()
            .py_2()
            .rounded(px(8.0))
            .bg(cx.theme().primary)
            .hover(|el| el.bg(cx.theme().primary_hover))
            .text_color(cx.theme().primary_foreground)
            .child(if self.label.is_empty() {
                "Button".into()
            } else {
                self.label
            })
    }
}
