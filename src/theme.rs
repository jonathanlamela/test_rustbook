use gpui::App;
use gpui_component::{Theme, ThemeConfig, ThemeConfigColors, ThemeMode};
use std::rc::Rc;

macro_rules! colors {
    ($($field:ident: $value:expr),* $(,)?) => {{
        let mut colors = ThemeConfigColors::default();
        $(colors.$field = Some($value.into());)*
        colors
    }};
}

pub(crate) fn apply(cx: &mut App) {
    let mut theme = Theme::default();
    theme.light_theme = Rc::new(config("Light", ThemeMode::Light, colors_light()));
    theme.dark_theme = Rc::new(config("Dark", ThemeMode::Dark, colors_dark()));

    cx.set_global(theme);
    Theme::change(ThemeMode::Light, None, cx);
}

fn config(name: &str, mode: ThemeMode, colors: ThemeConfigColors) -> ThemeConfig {
    ThemeConfig {
        name: format!("test_rustbook {name}").into(),
        mode,
        radius: Some(8),
        shadow: Some(false),
        colors,
        ..Default::default()
    }
}

fn colors_dark() -> ThemeConfigColors {
    colors!(
        background: "#14141f",
        foreground: "#e6e6f0",
        border: "#2a2a3d",
        accent: "#343453",
        accent_foreground: "#ffffff",
        muted: "#1e1e2e",
        muted_foreground: "#9a9ab0",
        primary: "#7b57a7",
        primary_foreground: "#ffffff",
        primary_hover: "#644788",
        secondary: "#2a2a3d",
        secondary_foreground: "#c4c4da",
        input: "#343453",
        list: "#1a1a28",
        list_hover: "#242438",
        list_active: "#3a3a5c",
        scrollbar_thumb: "#3a3a5c",
        scrollbar_thumb_hover: "#4a4a74",
    )
}

fn colors_light() -> ThemeConfigColors {
    colors!(
        background: "#fbfbfe",
        foreground: "#17172b",
        border: "#e4e4f0",
        accent: "#dcdcff",
        accent_foreground: "#0b0b1a",
        muted: "#eeeeff",
        muted_foreground: "#5c5c78",
        primary: "#7b57a7",
        primary_foreground: "#ffffff",
        primary_hover: "#644788",
        secondary: "#e4e4f0",
        secondary_foreground: "#3a3a55",
        input: "#d4d4e6",
        list: "#ffffff",
        list_hover: "#f0f0ff",
        list_active: "#dcdcff",
        scrollbar_thumb: "#d4d4e6",
        scrollbar_thumb_hover: "#bebedc",
    )
}
