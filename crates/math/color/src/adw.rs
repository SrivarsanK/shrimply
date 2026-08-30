use crate::Color;

impl Color<f32> {
    pub const BLUE1: Self = Self::from_rgb8(0x99, 0xc1, 0xf1);
    pub const BLUE2: Self = Self::from_rgb8(0x62, 0xa0, 0xea);
    pub const BLUE3: Self = Self::from_rgb8(0x35, 0x84, 0xe4);
    pub const BLUE4: Self = Self::from_rgb8(0x1c, 0x71, 0xd8);
    pub const BLUE5: Self = Self::from_rgb8(0x1a, 0x5f, 0xb4);

    pub const GREEN1: Self = Self::from_rgb8(0x8f, 0xf0, 0xa4);
    pub const GREEN2: Self = Self::from_rgb8(0x57, 0xe3, 0x89);
    pub const GREEN3: Self = Self::from_rgb8(0x33, 0xd1, 0x7a);
    pub const GREEN4: Self = Self::from_rgb8(0x2e, 0xc2, 0x7e);
    pub const GREEN5: Self = Self::from_rgb8(0x26, 0xa2, 0x69);

    pub const YELLOW1: Self = Self::from_rgb8(0xf9, 0xf0, 0x6b);
    pub const YELLOW2: Self = Self::from_rgb8(0xf8, 0xe4, 0x5c);
    pub const YELLOW3: Self = Self::from_rgb8(0xf6, 0xd3, 0x2d);
    pub const YELLOW4: Self = Self::from_rgb8(0xf5, 0xc2, 0x11);
    pub const YELLOW5: Self = Self::from_rgb8(0xe5, 0xa5, 0x0a);

    pub const ORANGE1: Self = Self::from_rgb8(0xff, 0xbe, 0x6f);
    pub const ORANGE2: Self = Self::from_rgb8(0xff, 0xa3, 0x48);
    pub const ORANGE3: Self = Self::from_rgb8(0xff, 0x78, 0x00);
    pub const ORANGE4: Self = Self::from_rgb8(0xe6, 0x61, 0x00);
    pub const ORANGE5: Self = Self::from_rgb8(0xc6, 0x46, 0x00);

    pub const RED1: Self = Self::from_rgb8(0xf6, 0x61, 0x51);
    pub const RED2: Self = Self::from_rgb8(0xed, 0x33, 0x3b);
    pub const RED3: Self = Self::from_rgb8(0xe0, 0x1b, 0x24);
    pub const RED4: Self = Self::from_rgb8(0xc0, 0x1c, 0x28);
    pub const RED5: Self = Self::from_rgb8(0xa5, 0x1d, 0x2d);

    pub const PURPLE1: Self = Self::from_rgb8(0xdc, 0x8a, 0xdd);
    pub const PURPLE2: Self = Self::from_rgb8(0xc0, 0x61, 0xcb);
    pub const PURPLE3: Self = Self::from_rgb8(0x91, 0x41, 0xac);
    pub const PURPLE4: Self = Self::from_rgb8(0x81, 0x3d, 0x9c);
    pub const PURPLE5: Self = Self::from_rgb8(0x61, 0x35, 0x83);

    pub const BROWN1: Self = Self::from_rgb8(0xcd, 0xab, 0x8f);
    pub const BROWN2: Self = Self::from_rgb8(0xb5, 0x83, 0x5a);
    pub const BROWN3: Self = Self::from_rgb8(0x98, 0x6a, 0x44);
    pub const BROWN4: Self = Self::from_rgb8(0x86, 0x5e, 0x3c);
    pub const BROWN5: Self = Self::from_rgb8(0x63, 0x45, 0x2c);

    pub const LIGHT1: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const LIGHT2: Self = Self::from_rgb8(0xf6, 0xf5, 0xf4);
    pub const LIGHT3: Self = Self::from_rgb8(0xde, 0xdd, 0xda);
    pub const LIGHT4: Self = Self::from_rgb8(0xc0, 0xbf, 0xbc);
    pub const LIGHT5: Self = Self::from_rgb8(0x9a, 0x99, 0x96);

    pub const DARK1: Self = Self::from_rgb8(0x77, 0x76, 0x7b);
    pub const DARK2: Self = Self::from_rgb8(0x5e, 0x5c, 0x64);
    pub const DARK3: Self = Self::from_rgb8(0x3d, 0x38, 0x46);
    pub const DARK4: Self = Self::from_rgb8(0x24, 0x1f, 0x31);
    pub const DARK5: Self = Self::from_rgb8(0x00, 0x00, 0x00);
}

impl Color<f32> {
    pub const ACCENT_FG: Self = Self::from_rgb8(0xff, 0xff, 0xff);

    pub const ACCENT_BLUE: Self = Self::from_rgb8(0x35, 0x84, 0xe4);
    pub const ACCENT_TEAL: Self = Self::from_rgb8(0x21, 0x90, 0xa4);
    pub const ACCENT_GREEN: Self = Self::from_rgb8(0x3a, 0x94, 0x4a);
    pub const ACCENT_YELLOW: Self = Self::from_rgb8(0xc8, 0x88, 0x00);
    pub const ACCENT_ORANGE: Self = Self::from_rgb8(0xed, 0x5b, 0x00);
    pub const ACCENT_RED: Self = Self::from_rgb8(0xe6, 0x2d, 0x42);
    pub const ACCENT_PINK: Self = Self::from_rgb8(0xd5, 0x61, 0x99);
    pub const ACCENT_PURPLE: Self = Self::from_rgb8(0x91, 0x41, 0xac);
    pub const ACCENT_SLATE: Self = Self::from_rgb8(0x6f, 0x83, 0x96);

    pub const ACCENT_BLUE_STANDALONE_LIGHT: Self = Self::from_rgb8(0x04, 0x61, 0xbe);
    pub const ACCENT_BLUE_STANDALONE_DARK: Self = Self::from_rgb8(0x81, 0xd0, 0xff);
    pub const ACCENT_TEAL_STANDALONE_LIGHT: Self = Self::from_rgb8(0x00, 0x71, 0x84);
    pub const ACCENT_TEAL_STANDALONE_DARK: Self = Self::from_rgb8(0x7b, 0xdf, 0xf4);
    pub const ACCENT_GREEN_STANDALONE_LIGHT: Self = Self::from_rgb8(0x15, 0x77, 0x2e);
    pub const ACCENT_GREEN_STANDALONE_DARK: Self = Self::from_rgb8(0x8d, 0xe6, 0x98);
    pub const ACCENT_YELLOW_STANDALONE_LIGHT: Self = Self::from_rgb8(0x90, 0x53, 0x00);
    pub const ACCENT_YELLOW_STANDALONE_DARK: Self = Self::from_rgb8(0xff, 0xc0, 0x57);
    pub const ACCENT_ORANGE_STANDALONE_LIGHT: Self = Self::from_rgb8(0xb6, 0x22, 0x00);
    pub const ACCENT_ORANGE_STANDALONE_DARK: Self = Self::from_rgb8(0xff, 0x9c, 0x5b);
    pub const ACCENT_RED_STANDALONE_LIGHT: Self = Self::from_rgb8(0xc0, 0x00, 0x23);
    pub const ACCENT_RED_STANDALONE_DARK: Self = Self::from_rgb8(0xff, 0x88, 0x8c);
    pub const ACCENT_PINK_STANDALONE_LIGHT: Self = Self::from_rgb8(0xa2, 0x32, 0x6c);
    pub const ACCENT_PINK_STANDALONE_DARK: Self = Self::from_rgb8(0xff, 0xa0, 0xd8);
    pub const ACCENT_PURPLE_STANDALONE_LIGHT: Self = Self::from_rgb8(0x89, 0x39, 0xa4);
    pub const ACCENT_PURPLE_STANDALONE_DARK: Self = Self::from_rgb8(0xfb, 0xa7, 0xff);
    pub const ACCENT_SLATE_STANDALONE_LIGHT: Self = Self::from_rgb8(0x52, 0x66, 0x78);
    pub const ACCENT_SLATE_STANDALONE_DARK: Self = Self::from_rgb8(0xbb, 0xd1, 0xe5);
}

impl Color<f32> {
    pub const DESTRUCTIVE_BG_LIGHT: Self = Self::from_rgb8(0xe0, 0x1b, 0x24);
    pub const DESTRUCTIVE_BG_DARK: Self = Self::from_rgb8(0xc0, 0x1c, 0x28);
    pub const DESTRUCTIVE_FG_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const DESTRUCTIVE_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const DESTRUCTIVE_LIGHT: Self = Self::from_rgb8(0xc3, 0x00, 0x00);
    pub const DESTRUCTIVE_DARK: Self = Self::from_rgb8(0xff, 0x93, 0x8c);

    pub const SUCCESS_BG_LIGHT: Self = Self::from_rgb8(0x2e, 0xc2, 0x7e);
    pub const SUCCESS_BG_DARK: Self = Self::from_rgb8(0x26, 0xa2, 0x69);
    pub const SUCCESS_FG_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const SUCCESS_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const SUCCESS_LIGHT: Self = Self::from_rgb8(0x00, 0x7c, 0x3d);
    pub const SUCCESS_DARK: Self = Self::from_rgb8(0x78, 0xe9, 0xab);

    pub const WARNING_BG_LIGHT: Self = Self::from_rgb8(0xe5, 0xa5, 0x0a);
    pub const WARNING_BG_DARK: Self = Self::from_rgb8(0xcd, 0x93, 0x09);
    pub const WARNING_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x00, 0.80);
    pub const WARNING_FG_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x00, 0.80);
    pub const WARNING_LIGHT: Self = Self::from_rgb8(0x90, 0x54, 0x00);
    pub const WARNING_DARK: Self = Self::from_rgb8(0xff, 0xc2, 0x52);

    pub const ERROR_BG_LIGHT: Self = Self::from_rgb8(0xe0, 0x1b, 0x24);
    pub const ERROR_BG_DARK: Self = Self::from_rgb8(0xc0, 0x1c, 0x28);
    pub const ERROR_FG_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const ERROR_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const ERROR_LIGHT: Self = Self::from_rgb8(0xc3, 0x00, 0x00);
    pub const ERROR_DARK: Self = Self::from_rgb8(0xff, 0x93, 0x8c);

    pub const WINDOW_BG_LIGHT: Self = Self::from_rgb8(0xfa, 0xfa, 0xfb);
    pub const WINDOW_BG_DARK: Self = Self::from_rgb8(0x22, 0x22, 0x26);
    pub const WINDOW_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const WINDOW_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);

    pub const VIEW_BG_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const VIEW_BG_DARK: Self = Self::from_rgb8(0x1d, 0x1d, 0x20);
    pub const VIEW_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const VIEW_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);

    pub const HEADERBAR_BG_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const HEADERBAR_BG_DARK: Self = Self::from_rgb8(0x2e, 0x2e, 0x32);
    pub const HEADERBAR_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const HEADERBAR_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const HEADERBAR_BORDER_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const HEADERBAR_BORDER_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const HEADERBAR_BACKDROP_LIGHT: Self = Self::from_rgb8(0xfa, 0xfa, 0xfb);
    pub const HEADERBAR_BACKDROP_DARK: Self = Self::from_rgb8(0x22, 0x22, 0x26);
    pub const HEADERBAR_SHADE_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.12);
    pub const HEADERBAR_SHADE_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.36);
    pub const HEADERBAR_DARKER_SHADE_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.12);
    pub const HEADERBAR_DARKER_SHADE_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x0c, 0.90);

    pub const SIDEBAR_BG_LIGHT: Self = Self::from_rgb8(0xeb, 0xeb, 0xed);
    pub const SIDEBAR_BG_DARK: Self = Self::from_rgb8(0x2e, 0x2e, 0x32);
    pub const SIDEBAR_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const SIDEBAR_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const SIDEBAR_BACKDROP_LIGHT: Self = Self::from_rgb8(0xf2, 0xf2, 0xf4);
    pub const SIDEBAR_BACKDROP_DARK: Self = Self::from_rgb8(0x28, 0x28, 0x2c);
    pub const SIDEBAR_BORDER_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.07);
    pub const SIDEBAR_BORDER_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.36);
    pub const SIDEBAR_SHADE_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.07);
    pub const SIDEBAR_SHADE_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.25);

    pub const SECONDARY_SIDEBAR_BG_LIGHT: Self = Self::from_rgb8(0xf3, 0xf3, 0xf5);
    pub const SECONDARY_SIDEBAR_BG_DARK: Self = Self::from_rgb8(0x28, 0x28, 0x2c);
    pub const SECONDARY_SIDEBAR_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const SECONDARY_SIDEBAR_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const SECONDARY_SIDEBAR_BACKDROP_LIGHT: Self = Self::from_rgb8(0xf6, 0xf6, 0xfa);
    pub const SECONDARY_SIDEBAR_BACKDROP_DARK: Self = Self::from_rgb8(0x25, 0x25, 0x29);
    pub const SECONDARY_SIDEBAR_BORDER_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.07);
    pub const SECONDARY_SIDEBAR_BORDER_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.36);
    pub const SECONDARY_SIDEBAR_SHADE_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.07);
    pub const SECONDARY_SIDEBAR_SHADE_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.25);

    pub const CARD_BG_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const CARD_BG_DARK: Self = Self::from_rgb8_alpha(0xff, 0xff, 0xff, 0.08);
    pub const CARD_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const CARD_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const CARD_SHADE_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.07);
    pub const CARD_SHADE_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.36);

    pub const OVERVIEW_BG_LIGHT: Self = Self::from_rgb8(0xf3, 0xf3, 0xf5);
    pub const OVERVIEW_BG_DARK: Self = Self::from_rgb8(0x28, 0x28, 0x2c);
    pub const OVERVIEW_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const OVERVIEW_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);

    pub const THUMBNAIL_BG_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const THUMBNAIL_BG_DARK: Self = Self::from_rgb8(0x39, 0x39, 0x3d);
    pub const THUMBNAIL_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const THUMBNAIL_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);

    pub const ACTIVE_TOGGLE_BG_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const ACTIVE_TOGGLE_BG_DARK: Self = Self::from_rgb8_alpha(0xff, 0xff, 0xff, 0.20);
    pub const ACTIVE_TOGGLE_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const ACTIVE_TOGGLE_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);

    pub const DIALOG_BG_LIGHT: Self = Self::from_rgb8(0xfa, 0xfa, 0xfb);
    pub const DIALOG_BG_DARK: Self = Self::from_rgb8(0x36, 0x36, 0x3a);
    pub const DIALOG_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const DIALOG_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);

    pub const POPOVER_BG_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const POPOVER_BG_DARK: Self = Self::from_rgb8(0x36, 0x36, 0x3a);
    pub const POPOVER_FG_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.80);
    pub const POPOVER_FG_DARK: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const POPOVER_SHADE_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.07);
    pub const POPOVER_SHADE_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.25);

    pub const SHADE_LIGHT: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.07);
    pub const SHADE_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x06, 0.25);
    pub const SCROLLBAR_OUTLINE_LIGHT: Self = Self::from_rgb8(0xff, 0xff, 0xff);
    pub const SCROLLBAR_OUTLINE_DARK: Self = Self::from_rgb8_alpha(0x00, 0x00, 0x0c, 0.95);
}
