use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme as _;
use gpui_preview::Previewable;

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