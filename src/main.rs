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
    /// Corner rounding in pixels.
    #[preview(slider(min = 0.0, max = 24.0))]
    pub border_radius: f32,
}

#[derive(Clone, Default, Previewable)]
#[preview(category = "Inputs")]
pub struct Label {
    pub content: String,
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .text_color(cx.theme().secondary_foreground)
            .child(if self.content.is_empty() {
                "Demo text".into()
            } else {
                self.content
            })
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .px_4()
            .py_2()
            .rounded(px(self.border_radius))
            .bg(cx.theme().primary)
            .text_color(cx.theme().primary_foreground)
            .child(if self.label.is_empty() {
                "Button".into()
            } else {
                self.label
            })
    }
}

fn main() {
    gpui_preview::run_with_assets(gpui_component_assets::Assets);
}
