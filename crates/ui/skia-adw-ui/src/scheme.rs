//! Theme-aware access to the Adwaita palette for custom-drawn UI.

use shrimply_math_color::Color;

fn dark_scheme() -> bool {
    adw::StyleManager::default().is_dark()
}

pub fn view_bg() -> Color {
    if dark_scheme() {
        Color::VIEW_BG_DARK
    } else {
        Color::VIEW_BG_LIGHT
    }
}

pub fn view_fg() -> Color {
    if dark_scheme() {
        Color::VIEW_FG_DARK
    } else {
        Color::VIEW_FG_LIGHT
    }
}

pub fn sidebar_border() -> Color {
    if dark_scheme() {
        Color::SIDEBAR_BORDER_DARK
    } else {
        Color::SIDEBAR_BORDER_LIGHT
    }
}

pub fn sidebar_shade() -> Color {
    if dark_scheme() {
        Color::SIDEBAR_SHADE_DARK
    } else {
        Color::SIDEBAR_SHADE_LIGHT
    }
}
