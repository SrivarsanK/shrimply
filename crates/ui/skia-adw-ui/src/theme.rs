//! Adwaita's complete light and dark palettes for custom-drawn UI, mirrored
//! from `libadwaita/src/stylesheet/_colors.scss`.

use shrimply_math_color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub accent_blue_standalone: Color,
    pub accent_teal_standalone: Color,
    pub accent_green_standalone: Color,
    pub accent_yellow_standalone: Color,
    pub accent_orange_standalone: Color,
    pub accent_red_standalone: Color,
    pub accent_pink_standalone: Color,
    pub accent_purple_standalone: Color,
    pub accent_slate_standalone: Color,
    pub destructive_bg: Color,
    pub destructive_fg: Color,
    pub destructive: Color,
    pub success_bg: Color,
    pub success_fg: Color,
    pub success: Color,
    pub warning_bg: Color,
    pub warning_fg: Color,
    pub warning: Color,
    pub error_bg: Color,
    pub error_fg: Color,
    pub error: Color,
    pub window_bg: Color,
    pub window_fg: Color,
    pub view_bg: Color,
    pub view_fg: Color,
    pub headerbar_bg: Color,
    pub headerbar_fg: Color,
    pub headerbar_border: Color,
    pub headerbar_backdrop: Color,
    pub headerbar_shade: Color,
    pub headerbar_darker_shade: Color,
    pub sidebar_bg: Color,
    pub sidebar_fg: Color,
    pub sidebar_backdrop: Color,
    pub sidebar_border: Color,
    pub sidebar_shade: Color,
    pub secondary_sidebar_bg: Color,
    pub secondary_sidebar_fg: Color,
    pub secondary_sidebar_backdrop: Color,
    pub secondary_sidebar_border: Color,
    pub secondary_sidebar_shade: Color,
    pub card_bg: Color,
    pub card_fg: Color,
    pub card_shade: Color,
    pub dialog_bg: Color,
    pub dialog_fg: Color,
    pub popover_bg: Color,
    pub popover_fg: Color,
    pub popover_shade: Color,
    pub thumbnail_bg: Color,
    pub thumbnail_fg: Color,
    pub shade: Color,
    pub scrollbar_outline: Color,
    pub active_toggle_bg: Color,
    pub active_toggle_fg: Color,
    pub overview_bg: Color,
    pub overview_fg: Color,
}

pub static LIGHT: Theme = Theme::new(false);
pub static DARK: Theme = Theme::new(true);

std::thread_local! {
    static STYLE_MANAGER: adw::StyleManager = adw::StyleManager::default();
}

pub fn current() -> &'static Theme {
    if STYLE_MANAGER.with(adw::StyleManager::is_dark) {
        &DARK
    } else {
        &LIGHT
    }
}

impl Theme {
    const fn new(dark: bool) -> Self {
        Self {
            accent_blue_standalone: select(
                dark,
                Color::ACCENT_BLUE_STANDALONE_LIGHT,
                Color::ACCENT_BLUE_STANDALONE_DARK,
            ),
            accent_teal_standalone: select(
                dark,
                Color::ACCENT_TEAL_STANDALONE_LIGHT,
                Color::ACCENT_TEAL_STANDALONE_DARK,
            ),
            accent_green_standalone: select(
                dark,
                Color::ACCENT_GREEN_STANDALONE_LIGHT,
                Color::ACCENT_GREEN_STANDALONE_DARK,
            ),
            accent_yellow_standalone: select(
                dark,
                Color::ACCENT_YELLOW_STANDALONE_LIGHT,
                Color::ACCENT_YELLOW_STANDALONE_DARK,
            ),
            accent_orange_standalone: select(
                dark,
                Color::ACCENT_ORANGE_STANDALONE_LIGHT,
                Color::ACCENT_ORANGE_STANDALONE_DARK,
            ),
            accent_red_standalone: select(
                dark,
                Color::ACCENT_RED_STANDALONE_LIGHT,
                Color::ACCENT_RED_STANDALONE_DARK,
            ),
            accent_pink_standalone: select(
                dark,
                Color::ACCENT_PINK_STANDALONE_LIGHT,
                Color::ACCENT_PINK_STANDALONE_DARK,
            ),
            accent_purple_standalone: select(
                dark,
                Color::ACCENT_PURPLE_STANDALONE_LIGHT,
                Color::ACCENT_PURPLE_STANDALONE_DARK,
            ),
            accent_slate_standalone: select(
                dark,
                Color::ACCENT_SLATE_STANDALONE_LIGHT,
                Color::ACCENT_SLATE_STANDALONE_DARK,
            ),
            destructive_bg: select(
                dark,
                Color::DESTRUCTIVE_BG_LIGHT,
                Color::DESTRUCTIVE_BG_DARK,
            ),
            destructive_fg: select(
                dark,
                Color::DESTRUCTIVE_FG_LIGHT,
                Color::DESTRUCTIVE_FG_DARK,
            ),
            destructive: select(dark, Color::DESTRUCTIVE_LIGHT, Color::DESTRUCTIVE_DARK),
            success_bg: select(dark, Color::SUCCESS_BG_LIGHT, Color::SUCCESS_BG_DARK),
            success_fg: select(dark, Color::SUCCESS_FG_LIGHT, Color::SUCCESS_FG_DARK),
            success: select(dark, Color::SUCCESS_LIGHT, Color::SUCCESS_DARK),
            warning_bg: select(dark, Color::WARNING_BG_LIGHT, Color::WARNING_BG_DARK),
            warning_fg: select(dark, Color::WARNING_FG_LIGHT, Color::WARNING_FG_DARK),
            warning: select(dark, Color::WARNING_LIGHT, Color::WARNING_DARK),
            error_bg: select(dark, Color::ERROR_BG_LIGHT, Color::ERROR_BG_DARK),
            error_fg: select(dark, Color::ERROR_FG_LIGHT, Color::ERROR_FG_DARK),
            error: select(dark, Color::ERROR_LIGHT, Color::ERROR_DARK),
            window_bg: select(dark, Color::WINDOW_BG_LIGHT, Color::WINDOW_BG_DARK),
            window_fg: select(dark, Color::WINDOW_FG_LIGHT, Color::WINDOW_FG_DARK),
            view_bg: select(dark, Color::VIEW_BG_LIGHT, Color::VIEW_BG_DARK),
            view_fg: select(dark, Color::VIEW_FG_LIGHT, Color::VIEW_FG_DARK),
            headerbar_bg: select(dark, Color::HEADERBAR_BG_LIGHT, Color::HEADERBAR_BG_DARK),
            headerbar_fg: select(dark, Color::HEADERBAR_FG_LIGHT, Color::HEADERBAR_FG_DARK),
            headerbar_border: select(
                dark,
                Color::HEADERBAR_BORDER_LIGHT,
                Color::HEADERBAR_BORDER_DARK,
            ),
            headerbar_backdrop: select(
                dark,
                Color::HEADERBAR_BACKDROP_LIGHT,
                Color::HEADERBAR_BACKDROP_DARK,
            ),
            headerbar_shade: select(
                dark,
                Color::HEADERBAR_SHADE_LIGHT,
                Color::HEADERBAR_SHADE_DARK,
            ),
            headerbar_darker_shade: select(
                dark,
                Color::HEADERBAR_DARKER_SHADE_LIGHT,
                Color::HEADERBAR_DARKER_SHADE_DARK,
            ),
            sidebar_bg: select(dark, Color::SIDEBAR_BG_LIGHT, Color::SIDEBAR_BG_DARK),
            sidebar_fg: select(dark, Color::SIDEBAR_FG_LIGHT, Color::SIDEBAR_FG_DARK),
            sidebar_backdrop: select(
                dark,
                Color::SIDEBAR_BACKDROP_LIGHT,
                Color::SIDEBAR_BACKDROP_DARK,
            ),
            sidebar_border: select(
                dark,
                Color::SIDEBAR_BORDER_LIGHT,
                Color::SIDEBAR_BORDER_DARK,
            ),
            sidebar_shade: select(dark, Color::SIDEBAR_SHADE_LIGHT, Color::SIDEBAR_SHADE_DARK),
            secondary_sidebar_bg: select(
                dark,
                Color::SECONDARY_SIDEBAR_BG_LIGHT,
                Color::SECONDARY_SIDEBAR_BG_DARK,
            ),
            secondary_sidebar_fg: select(
                dark,
                Color::SECONDARY_SIDEBAR_FG_LIGHT,
                Color::SECONDARY_SIDEBAR_FG_DARK,
            ),
            secondary_sidebar_backdrop: select(
                dark,
                Color::SECONDARY_SIDEBAR_BACKDROP_LIGHT,
                Color::SECONDARY_SIDEBAR_BACKDROP_DARK,
            ),
            secondary_sidebar_border: select(
                dark,
                Color::SECONDARY_SIDEBAR_BORDER_LIGHT,
                Color::SECONDARY_SIDEBAR_BORDER_DARK,
            ),
            secondary_sidebar_shade: select(
                dark,
                Color::SECONDARY_SIDEBAR_SHADE_LIGHT,
                Color::SECONDARY_SIDEBAR_SHADE_DARK,
            ),
            card_bg: select(dark, Color::CARD_BG_LIGHT, Color::CARD_BG_DARK),
            card_fg: select(dark, Color::CARD_FG_LIGHT, Color::CARD_FG_DARK),
            card_shade: select(dark, Color::CARD_SHADE_LIGHT, Color::CARD_SHADE_DARK),
            dialog_bg: select(dark, Color::DIALOG_BG_LIGHT, Color::DIALOG_BG_DARK),
            dialog_fg: select(dark, Color::DIALOG_FG_LIGHT, Color::DIALOG_FG_DARK),
            popover_bg: select(dark, Color::POPOVER_BG_LIGHT, Color::POPOVER_BG_DARK),
            popover_fg: select(dark, Color::POPOVER_FG_LIGHT, Color::POPOVER_FG_DARK),
            popover_shade: select(dark, Color::POPOVER_SHADE_LIGHT, Color::POPOVER_SHADE_DARK),
            thumbnail_bg: select(dark, Color::THUMBNAIL_BG_LIGHT, Color::THUMBNAIL_BG_DARK),
            thumbnail_fg: select(dark, Color::THUMBNAIL_FG_LIGHT, Color::THUMBNAIL_FG_DARK),
            shade: select(dark, Color::SHADE_LIGHT, Color::SHADE_DARK),
            scrollbar_outline: select(
                dark,
                Color::SCROLLBAR_OUTLINE_LIGHT,
                Color::SCROLLBAR_OUTLINE_DARK,
            ),
            active_toggle_bg: select(
                dark,
                Color::ACTIVE_TOGGLE_BG_LIGHT,
                Color::ACTIVE_TOGGLE_BG_DARK,
            ),
            active_toggle_fg: select(
                dark,
                Color::ACTIVE_TOGGLE_FG_LIGHT,
                Color::ACTIVE_TOGGLE_FG_DARK,
            ),
            overview_bg: select(dark, Color::OVERVIEW_BG_LIGHT, Color::OVERVIEW_BG_DARK),
            overview_fg: select(dark, Color::OVERVIEW_FG_LIGHT, Color::OVERVIEW_FG_DARK),
        }
    }
}

const fn select(dark: bool, light: Color, dark_color: Color) -> Color {
    if dark { dark_color } else { light }
}
