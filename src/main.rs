mod button;
mod label;
mod theme;

use gpui::prelude::*;
use gpui::*;
use gpui_preview::{PreviewApp, CloseDialog, SelectNext, SelectPrev};

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        theme::apply(cx);

        cx.bind_keys([
            KeyBinding::new("up", SelectPrev, None),
            KeyBinding::new("down", SelectNext, None),
            KeyBinding::new("escape", CloseDialog, None),
        ]);

        cx.on_window_closed(|cx| {
            cx.quit();
        })
        .detach();

        cx.activate(true);

        let window_size = size(px(1400.), px(900.));
        let bounds = Bounds::centered(None, window_size, cx);

        cx.spawn(async move |cx| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(800.), px(500.))),
                titlebar: Some(TitlebarOptions {
                    title: Some("gpui-preview".into()),
                    ..Default::default()
                }),
                ..Default::default()
            };

            cx.open_window(options, |window, cx| {
                let view = cx.new(|cx| PreviewApp::new(window, cx));
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}