pub enum LightDarkMode {
    LightMode,
    DarkMode,
}

pub enum ThemeName {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

///
pub fn set_theme(ctx: &egui::Context, theme_name: ThemeName) {
    match theme_name {
        ThemeName::Latte => {
            update_theme(ctx, &LATTE, LightDarkMode::LightMode);
        }
        ThemeName::Frappe => {
            update_theme(ctx, &FRAPPE, LightDarkMode::DarkMode);
        }
        ThemeName::Macchiato => {
            update_theme(ctx, &MACCHIATO, LightDarkMode::DarkMode);
        }
        ThemeName::Mocha => {
            update_theme(ctx, &MOCHA, LightDarkMode::DarkMode);
        }
    }
}

fn update_theme(ctx: &egui::Context, theme: &ThemeColors, light_dark: LightDarkMode) {
    ctx.set_visuals(egui::Visuals {
        dark_mode: match light_dark {
            LightDarkMode::DarkMode => false,
            LightDarkMode::LightMode => true,
        },
        override_text_color: Some(theme.text),
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: theme.base,
                weak_bg_fill: theme.base,
                bg_stroke: egui::Stroke {
                    color: theme.overlay1,
                    width: 1.0,
                },
                corner_radius: egui::CornerRadius::ZERO,
                fg_stroke: egui::Stroke {
                    color: theme.text,
                    width: 1.0,
                },
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: theme.surface0,
                weak_bg_fill: theme.surface0,
                bg_stroke: egui::Stroke {
                    color: theme.overlay1,
                    width: 1.0,
                },
                corner_radius: egui::CornerRadius::ZERO,
                fg_stroke: egui::Stroke {
                    color: theme.text,
                    width: 1.0,
                },
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: theme.surface2,
                weak_bg_fill: theme.surface2,
                bg_stroke: egui::Stroke {
                    color: theme.overlay1,
                    width: 1.0,
                },
                corner_radius: egui::CornerRadius::ZERO, // egui::Rounding of highlight around button or label
                fg_stroke: egui::Stroke {
                    color: theme.text,
                    width: 1.0,
                },
                expansion: 0.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: theme.surface1,
                weak_bg_fill: theme.surface1,
                bg_stroke: egui::Stroke {
                    color: theme.overlay1,
                    width: 1.0,
                },
                corner_radius: egui::CornerRadius::ZERO,
                fg_stroke: egui::Stroke {
                    color: theme.text,
                    width: 1.0,
                },
                expansion: 0.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: theme.surface0,
                weak_bg_fill: theme.surface0,
                bg_stroke: egui::Stroke {
                    color: theme.overlay1,
                    width: 1.0,
                },
                corner_radius: egui::CornerRadius::ZERO,
                fg_stroke: egui::Stroke {
                    color: theme.text,
                    width: 1.0,
                },
                expansion: 0.0,
            },
        },
        selection: egui::style::Selection {
            bg_fill: theme.blue.linear_multiply(match light_dark {
                LightDarkMode::DarkMode => 0.4,
                LightDarkMode::LightMode => 0.2,
            }),
            stroke: egui::Stroke {
                color: theme.overlay1,
                width: 1.0,
            },
        },
        hyperlink_color: theme.rosewater,
        faint_bg_color: theme.surface0,
        extreme_bg_color: theme.crust,
        code_bg_color: theme.mantle,
        warn_fg_color: theme.peach,
        error_fg_color: theme.maroon,
        window_corner_radius: egui::CornerRadius::ZERO,
        window_shadow: egui::epaint::Shadow {
            color: theme.base,
            offset: [0, 0],
            blur: 0,
            spread: 0,
        },
        window_fill: theme.base,
        window_stroke: egui::Stroke {
            color: theme.overlay1,
            width: 1.0,
        },
        window_highlight_topmost: true,
        menu_corner_radius: egui::CornerRadius::same(4), // Menu dropdown and tooltip "on_hover_text" rounding
        panel_fill: theme.base,
        popup_shadow: egui::epaint::Shadow {
            color: theme.base,
            offset: [0, 0],
            blur: 0,
            spread: 0,
        },
        resize_corner_size: 0.0,
        text_cursor: egui::style::TextCursorStyle {
            stroke: egui::Stroke {
                width: 2.0,
                color: theme.overlay1,
            },
            preview: false,
            blink: true,
            on_duration: 0.5,
            off_duration: 0.5,
        },
        clip_rect_margin: 0.0,
        button_frame: true,
        collapsing_header_frame: true,
        indent_has_left_vline: true,
        striped: true,
        slider_trailing_fill: true,
        handle_shape: egui::style::HandleShape::Circle,
        interact_cursor: Some(egui::CursorIcon::Default),
        image_loading_spinners: true,
        numeric_color_space: egui::style::NumericColorSpace::Linear,
    });
}

#[derive(Debug, Default)]
pub struct ThemeColors {
    pub rosewater: egui::Color32,
    pub maroon: egui::Color32,
    pub peach: egui::Color32,
    pub blue: egui::Color32,
    pub text: egui::Color32,
    pub overlay1: egui::Color32,
    pub surface2: egui::Color32,
    pub surface1: egui::Color32,
    pub surface0: egui::Color32, // faint_bg_color (button, checkbox backgrounds)
    pub base: egui::Color32,     // Panel background
    pub mantle: egui::Color32,
    pub crust: egui::Color32, // extreme_bg_color (TextEdit boxes)
}

pub const LATTE: ThemeColors = ThemeColors {
    rosewater: egui::Color32::from_rgb(220, 138, 120),
    maroon: egui::Color32::from_rgb(230, 69, 83),
    peach: egui::Color32::from_rgb(254, 100, 11),
    blue: egui::Color32::from_rgb(30, 102, 245),
    text: egui::Color32::from_rgb(76, 79, 105),
    overlay1: egui::Color32::from_rgb(140, 143, 161),
    surface2: egui::Color32::from_rgb(172, 176, 190),
    surface1: egui::Color32::from_rgb(188, 192, 204),
    surface0: egui::Color32::from_rgb(204, 208, 218),
    base: egui::Color32::from_rgb(239, 241, 245),
    mantle: egui::Color32::from_rgb(230, 233, 239),
    crust: egui::Color32::from_rgb(220, 224, 232),
};

pub const FRAPPE: ThemeColors = ThemeColors {
    rosewater: egui::Color32::from_rgb(242, 213, 207),
    maroon: egui::Color32::from_rgb(234, 153, 156),
    peach: egui::Color32::from_rgb(239, 159, 118),
    blue: egui::Color32::from_rgb(140, 170, 238),
    text: egui::Color32::from_rgb(198, 208, 245),
    overlay1: egui::Color32::from_rgb(131, 139, 167),
    surface2: egui::Color32::from_rgb(98, 104, 128),
    surface1: egui::Color32::from_rgb(81, 87, 109),
    surface0: egui::Color32::from_rgb(65, 69, 89),
    base: egui::Color32::from_rgb(48, 52, 70),
    mantle: egui::Color32::from_rgb(41, 44, 60),
    crust: egui::Color32::from_rgb(35, 38, 52),
};

pub const MACCHIATO: ThemeColors = ThemeColors {
    rosewater: egui::Color32::from_rgb(244, 219, 214),
    maroon: egui::Color32::from_rgb(238, 153, 160),
    peach: egui::Color32::from_rgb(245, 169, 127),
    blue: egui::Color32::from_rgb(138, 173, 244),
    text: egui::Color32::from_rgb(202, 211, 245),
    overlay1: egui::Color32::from_rgb(128, 135, 162),
    surface2: egui::Color32::from_rgb(91, 96, 120),
    surface1: egui::Color32::from_rgb(73, 77, 100),
    surface0: egui::Color32::from_rgb(54, 58, 79),
    base: egui::Color32::from_rgb(36, 39, 58),
    mantle: egui::Color32::from_rgb(30, 32, 48),
    crust: egui::Color32::from_rgb(24, 25, 38),
};

pub const MOCHA: ThemeColors = ThemeColors {
    rosewater: egui::Color32::from_rgb(245, 224, 220),
    maroon: egui::Color32::from_rgb(235, 160, 172),
    peach: egui::Color32::from_rgb(250, 179, 135),
    blue: egui::Color32::from_rgb(137, 180, 250),
    text: egui::Color32::from_rgb(205, 214, 244),
    overlay1: egui::Color32::from_rgb(127, 132, 156),
    surface2: egui::Color32::from_rgb(88, 91, 112),
    surface1: egui::Color32::from_rgb(69, 71, 90),
    surface0: egui::Color32::from_rgb(49, 50, 68),
    base: egui::Color32::from_rgb(30, 30, 46),
    mantle: egui::Color32::from_rgb(24, 24, 37),
    crust: egui::Color32::from_rgb(17, 17, 27),
};
