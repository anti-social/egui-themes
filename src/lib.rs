#[derive(Debug, Default)]
pub enum State {
    Latte,
    #[default]
    Frappe,
    Macchiato,
    Mocha,
}

#[derive(Debug, Default)]
pub struct StateMachine {
    state: State,
    theme: Theme,
    is_latte: bool, // Used for setting ctx.set_visuals light or dark mode
}

impl StateMachine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_theme(&mut self, ctx: &egui::Context, theme: &Theme) {
        StateMachine::update_theme(self, ctx, theme);
    }

    pub fn rotate_theme(&mut self, ctx: &egui::Context) {
        match self.state {
            State::Latte => {
                self.theme = FRAPPE;
                self.state = State::Frappe;
                self.is_latte = false;
            }
            State::Frappe => {
                self.theme = MACCHIATO;
                self.state = State::Macchiato;
                self.is_latte = false;
            }
            State::Macchiato => {
                self.theme = MOCHA;
                self.state = State::Mocha;
                self.is_latte = false;
            }
            State::Mocha => {
                self.theme = LATTE;
                self.state = State::Latte;
                self.is_latte = true;
            }
        }
        StateMachine::update_theme(self, ctx, &self.theme);
    }

    fn update_theme(&self, ctx: &egui::Context, theme: &Theme) {
        ctx.set_visuals(egui::Visuals {
            dark_mode: if self.is_latte { false } else { true },
            override_text_color: Some(theme.text),
            widgets: egui::style::Widgets {
                noninteractive: egui::style::WidgetVisuals {
                    bg_fill: theme.base,
                    weak_bg_fill: theme.base,
                    bg_stroke: egui::Stroke {
                        color: theme.overlay1,
                        width: 1.0,
                    },
                    rounding: egui::Rounding::ZERO,
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
                    rounding: egui::Rounding::ZERO,
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
                    rounding: egui::Rounding::ZERO, // egui::Rounding of highlight around button or label
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
                    rounding: egui::Rounding::ZERO,
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
                    rounding: egui::Rounding::ZERO,
                    fg_stroke: egui::Stroke {
                        color: theme.text,
                        width: 1.0,
                    },
                    expansion: 0.0,
                },
            },
            selection: egui::style::Selection {
                bg_fill: theme
                    .blue
                    .linear_multiply(if self.is_latte { 0.4 } else { 0.2 }),
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
            window_rounding: egui::Rounding::ZERO,
            window_shadow: egui::epaint::Shadow {
                color: theme.base,
                offset: egui::Vec2 { x: 0.0, y: 0.0 },
                blur: 0.0,
                spread: 0.0,
            },
            window_fill: theme.base,
            window_stroke: egui::Stroke {
                color: theme.overlay1,
                width: 1.0,
            },
            window_highlight_topmost: true,
            menu_rounding: egui::Rounding::same(4.), // Menu dropdown and tooltip "on_hover_text" rounding
            panel_fill: theme.base,
            popup_shadow: egui::epaint::Shadow {
                color: theme.base,
                offset: egui::Vec2 { x: 0.0, y: 0.0 },
                blur: 0.0,
                spread: 0.0,
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
}

#[derive(Debug, Default)]
pub struct Theme {
    pub rosewater: egui::Color32,
    pub flamingo: egui::Color32,
    pub pink: egui::Color32,
    pub mauve: egui::Color32,
    pub red: egui::Color32,
    pub maroon: egui::Color32,
    pub peach: egui::Color32,
    pub yellow: egui::Color32,
    pub green: egui::Color32,
    pub teal: egui::Color32,
    pub sky: egui::Color32,
    pub sapphire: egui::Color32,
    pub blue: egui::Color32,
    pub lavender: egui::Color32,
    pub text: egui::Color32,
    pub subtext1: egui::Color32,
    pub subtext0: egui::Color32,
    pub overlay2: egui::Color32,
    pub overlay1: egui::Color32,
    pub overlay0: egui::Color32,
    pub surface2: egui::Color32,
    pub surface1: egui::Color32,
    pub surface0: egui::Color32,
    pub base: egui::Color32,
    pub mantle: egui::Color32,
    pub crust: egui::Color32,
}

pub const LATTE: Theme = Theme {
    rosewater: egui::Color32::from_rgb(220, 138, 120),
    flamingo: egui::Color32::from_rgb(221, 120, 120),
    pink: egui::Color32::from_rgb(234, 118, 203),
    mauve: egui::Color32::from_rgb(136, 57, 239),
    red: egui::Color32::from_rgb(210, 15, 57),
    maroon: egui::Color32::from_rgb(230, 69, 83),
    peach: egui::Color32::from_rgb(254, 100, 11),
    yellow: egui::Color32::from_rgb(223, 142, 29),
    green: egui::Color32::from_rgb(64, 160, 43),
    teal: egui::Color32::from_rgb(23, 146, 153),
    sky: egui::Color32::from_rgb(4, 165, 229),
    sapphire: egui::Color32::from_rgb(32, 159, 181),
    blue: egui::Color32::from_rgb(30, 102, 245),
    lavender: egui::Color32::from_rgb(114, 135, 253),
    text: egui::Color32::from_rgb(76, 79, 105),
    subtext1: egui::Color32::from_rgb(92, 95, 119),
    subtext0: egui::Color32::from_rgb(108, 111, 133),
    overlay2: egui::Color32::from_rgb(124, 127, 147),
    overlay1: egui::Color32::from_rgb(140, 143, 161),
    overlay0: egui::Color32::from_rgb(156, 160, 176),
    surface2: egui::Color32::from_rgb(172, 176, 190),
    surface1: egui::Color32::from_rgb(188, 192, 204),
    surface0: egui::Color32::from_rgb(204, 208, 218),
    base: egui::Color32::from_rgb(239, 241, 245),
    mantle: egui::Color32::from_rgb(230, 233, 239),
    crust: egui::Color32::from_rgb(220, 224, 232),
};

pub const FRAPPE: Theme = Theme {
    rosewater: egui::Color32::from_rgb(242, 213, 207),
    flamingo: egui::Color32::from_rgb(238, 190, 190),
    pink: egui::Color32::from_rgb(244, 184, 228),
    mauve: egui::Color32::from_rgb(202, 158, 230),
    red: egui::Color32::from_rgb(231, 130, 132),
    maroon: egui::Color32::from_rgb(234, 153, 156),
    peach: egui::Color32::from_rgb(239, 159, 118),
    yellow: egui::Color32::from_rgb(229, 200, 144),
    green: egui::Color32::from_rgb(166, 209, 137),
    teal: egui::Color32::from_rgb(129, 200, 190),
    sky: egui::Color32::from_rgb(153, 209, 219),
    sapphire: egui::Color32::from_rgb(133, 193, 220),
    blue: egui::Color32::from_rgb(140, 170, 238),
    lavender: egui::Color32::from_rgb(186, 187, 241),
    text: egui::Color32::from_rgb(198, 208, 245),
    subtext1: egui::Color32::from_rgb(181, 191, 226),
    subtext0: egui::Color32::from_rgb(165, 173, 206),
    overlay2: egui::Color32::from_rgb(148, 156, 187),
    overlay1: egui::Color32::from_rgb(131, 139, 167),
    overlay0: egui::Color32::from_rgb(115, 121, 148),
    surface2: egui::Color32::from_rgb(98, 104, 128),
    surface1: egui::Color32::from_rgb(81, 87, 109),
    surface0: egui::Color32::from_rgb(65, 69, 89),
    base: egui::Color32::from_rgb(48, 52, 70),
    mantle: egui::Color32::from_rgb(41, 44, 60),
    crust: egui::Color32::from_rgb(35, 38, 52),
};

pub const MACCHIATO: Theme = Theme {
    rosewater: egui::Color32::from_rgb(244, 219, 214),
    flamingo: egui::Color32::from_rgb(240, 198, 198),
    pink: egui::Color32::from_rgb(245, 189, 230),
    mauve: egui::Color32::from_rgb(198, 160, 246),
    red: egui::Color32::from_rgb(237, 135, 150),
    maroon: egui::Color32::from_rgb(238, 153, 160),
    peach: egui::Color32::from_rgb(245, 169, 127),
    yellow: egui::Color32::from_rgb(238, 212, 159),
    green: egui::Color32::from_rgb(166, 218, 149),
    teal: egui::Color32::from_rgb(139, 213, 202),
    sky: egui::Color32::from_rgb(145, 215, 227),
    sapphire: egui::Color32::from_rgb(125, 196, 228),
    blue: egui::Color32::from_rgb(138, 173, 244),
    lavender: egui::Color32::from_rgb(183, 189, 248),
    text: egui::Color32::from_rgb(202, 211, 245),
    subtext1: egui::Color32::from_rgb(184, 192, 224),
    subtext0: egui::Color32::from_rgb(165, 173, 203),
    overlay2: egui::Color32::from_rgb(147, 154, 183),
    overlay1: egui::Color32::from_rgb(128, 135, 162),
    overlay0: egui::Color32::from_rgb(110, 115, 141),
    surface2: egui::Color32::from_rgb(91, 96, 120),
    surface1: egui::Color32::from_rgb(73, 77, 100),
    surface0: egui::Color32::from_rgb(54, 58, 79),
    base: egui::Color32::from_rgb(36, 39, 58),
    mantle: egui::Color32::from_rgb(30, 32, 48),
    crust: egui::Color32::from_rgb(24, 25, 38),
};

pub const MOCHA: Theme = Theme {
    rosewater: egui::Color32::from_rgb(245, 224, 220),
    flamingo: egui::Color32::from_rgb(242, 205, 205),
    pink: egui::Color32::from_rgb(245, 194, 231),
    mauve: egui::Color32::from_rgb(203, 166, 247),
    red: egui::Color32::from_rgb(243, 139, 168),
    maroon: egui::Color32::from_rgb(235, 160, 172),
    peach: egui::Color32::from_rgb(250, 179, 135),
    yellow: egui::Color32::from_rgb(249, 226, 175),
    green: egui::Color32::from_rgb(166, 227, 161),
    teal: egui::Color32::from_rgb(148, 226, 213),
    sky: egui::Color32::from_rgb(137, 220, 235),
    sapphire: egui::Color32::from_rgb(116, 199, 236),
    blue: egui::Color32::from_rgb(137, 180, 250),
    lavender: egui::Color32::from_rgb(180, 190, 254),
    text: egui::Color32::from_rgb(205, 214, 244),
    subtext1: egui::Color32::from_rgb(186, 194, 222),
    subtext0: egui::Color32::from_rgb(166, 173, 200),
    overlay2: egui::Color32::from_rgb(147, 153, 178),
    overlay1: egui::Color32::from_rgb(127, 132, 156),
    overlay0: egui::Color32::from_rgb(108, 112, 134),
    surface2: egui::Color32::from_rgb(88, 91, 112),
    surface1: egui::Color32::from_rgb(69, 71, 90),
    surface0: egui::Color32::from_rgb(49, 50, 68),
    base: egui::Color32::from_rgb(30, 30, 46),
    mantle: egui::Color32::from_rgb(24, 24, 37),
    crust: egui::Color32::from_rgb(17, 17, 27),
};
