use ratatui::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    // Background color (None means transparent/terminal default)
    pub background: Option<Color>,
    // Title colors
    pub title_try: Color,
    pub title_rs: Color,
    // Search box
    pub search_title: Color,
    pub search_border: Color,
    // Folder box
    pub folder_title: Color,
    pub folder_border: Color,
    // Disk box
    pub disk_title: Color,
    pub disk_border: Color,
    // Preview box
    pub preview_title: Color,
    pub preview_border: Color,
    // Legends box
    pub legends_title: Color,
    pub legends_border: Color,
    // List colors
    pub list_date: Color,
    pub list_highlight_bg: Color,
    pub list_highlight_fg: Color,
    pub list_selected_fg: Color,
    // Helpers/status bar
    pub helpers_colors: Color,
    pub status_message: Color,
    // Popup colors
    pub popup_bg: Color,
    pub popup_text: Color,
    // Icon colors
    pub icon_rust: Color,
    pub icon_maven: Color,
    pub icon_flutter: Color,
    pub icon_go: Color,
    pub icon_python: Color,
    pub icon_mise: Color,
    pub icon_worktree: Color,
    pub icon_worktree_lock: Color,
    pub icon_gitmodules: Color,
    pub icon_git: Color,
    pub icon_folder: Color,
    pub icon_file: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_theme()
    }
}

/// A palette of named base colors used to construct themes via standard mapping.
/// Most themes map these roles consistently:
///   accent1 → title_try, preview_title, icon_flutter
///   accent2 → title_rs, popup_text, icon_maven, icon_git
///   warm    → search_title, icon_rust, icon_mise
///   cool    → icon_go
///   green   → folder_title, icon_worktree
///   yellow  → disk_title, status_message, icon_python, icon_folder
///   purple  → legends_title, icon_gitmodules
///   overlay → all borders, helpers_colors
///   subtext → list_date, icon_worktree_lock, icon_file
///   surface → list_highlight_bg
///   text    → list_highlight_fg
///   base    → popup_bg
struct Palette {
    accent1: Color,
    accent2: Color,
    warm: Color,
    cool: Color,
    green: Color,
    yellow: Color,
    purple: Color,
    overlay: Color,
    subtext: Color,
    surface: Color,
    text: Color,
    base: Color,
}

impl Theme {
    /// Build a theme from a palette using the standard role mapping.
    fn from_palette(name: &str, background: Option<Color>, p: Palette) -> Self {
        Self {
            name: name.to_string(),
            background,
            title_try: p.accent1,
            title_rs: p.accent2,
            search_title: p.warm,
            search_border: p.overlay,
            folder_title: p.green,
            folder_border: p.overlay,
            disk_title: p.yellow,
            disk_border: p.overlay,
            preview_title: p.accent1,
            preview_border: p.overlay,
            legends_title: p.purple,
            legends_border: p.overlay,
            list_date: p.subtext,
            list_highlight_bg: p.surface,
            list_highlight_fg: p.text,
            list_selected_fg: p.text,
            helpers_colors: p.overlay,
            status_message: p.yellow,
            popup_bg: p.base,
            popup_text: p.accent2,
            icon_rust: p.warm,
            icon_maven: p.accent2,
            icon_flutter: p.accent1,
            icon_go: p.cool,
            icon_python: p.yellow,
            icon_mise: p.warm,
            icon_worktree: p.green,
            icon_worktree_lock: p.subtext,
            icon_gitmodules: p.purple,
            icon_git: p.accent2,
            icon_folder: p.yellow,
            icon_file: p.subtext,
        }
    }

    pub fn default_theme() -> Self {
        Self {
            // Default theme keeps original hardcoded icon colors for real-world brands
            icon_rust: Color::Rgb(230, 100, 50),
            icon_maven: Color::Rgb(255, 150, 50),
            icon_flutter: Color::Rgb(2, 123, 222),
            icon_go: Color::Rgb(0, 173, 216),
            icon_python: Color::Yellow,
            icon_mise: Color::Rgb(250, 179, 135),
            icon_worktree: Color::Rgb(100, 180, 100),
            icon_worktree_lock: Color::White,
            icon_gitmodules: Color::Rgb(180, 130, 200),
            icon_git: Color::Rgb(240, 80, 50),
            icon_folder: Color::Rgb(249, 226, 175),
            icon_file: Color::Rgb(166, 173, 200),
            ..Self::from_palette(
                "Default",
                None,
                Palette {
                    accent1: Color::Rgb(137, 180, 250), // Blue
                    accent2: Color::Rgb(243, 139, 168), // Red
                    warm: Color::Rgb(250, 179, 135),    // Peach
                    cool: Color::Rgb(0, 173, 216),      // Cyan
                    green: Color::Rgb(166, 227, 161),   // Green
                    yellow: Color::Rgb(249, 226, 175),  // Yellow
                    purple: Color::Rgb(203, 166, 247),  // Mauve
                    overlay: Color::Rgb(147, 153, 178), // Overlay
                    subtext: Color::Rgb(166, 173, 200), // Subtext
                    surface: Color::Rgb(88, 91, 112),   // Surface
                    text: Color::Rgb(205, 214, 244),    // Text
                    base: Color::Rgb(30, 30, 46),       // Base
                },
            )
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self::from_palette(
            "Catppuccin Mocha",
            Some(Color::Rgb(30, 30, 46)),
            Palette {
                accent1: Color::Rgb(137, 180, 250), // Blue
                accent2: Color::Rgb(243, 139, 168), // Red
                warm: Color::Rgb(250, 179, 135),    // Peach
                cool: Color::Rgb(148, 226, 213),    // Teal
                green: Color::Rgb(166, 227, 161),   // Green
                yellow: Color::Rgb(249, 226, 175),  // Yellow
                purple: Color::Rgb(203, 166, 247),  // Mauve
                overlay: Color::Rgb(147, 153, 178), // Overlay2
                subtext: Color::Rgb(166, 173, 200), // Subtext0
                surface: Color::Rgb(88, 91, 112),   // Surface2
                text: Color::Rgb(205, 214, 244),    // Text
                base: Color::Rgb(30, 30, 46),       // Base
            },
        )
    }

    pub fn catppuccin_macchiato() -> Self {
        Self {
            title_rs: Color::Rgb(238, 153, 160), // Maroon (not Red)
            ..Self::from_palette(
                "Catppuccin Macchiato",
                Some(Color::Rgb(36, 39, 58)),
                Palette {
                    accent1: Color::Rgb(138, 173, 244), // Blue
                    accent2: Color::Rgb(237, 135, 150), // Red
                    warm: Color::Rgb(245, 169, 127),    // Peach
                    cool: Color::Rgb(139, 213, 202),    // Teal
                    green: Color::Rgb(166, 218, 149),   // Green
                    yellow: Color::Rgb(238, 212, 159),  // Yellow
                    purple: Color::Rgb(198, 160, 246),  // Mauve
                    overlay: Color::Rgb(147, 154, 183), // Overlay1
                    subtext: Color::Rgb(165, 173, 203), // Subtext0
                    surface: Color::Rgb(91, 96, 120),   // Surface2
                    text: Color::Rgb(202, 211, 245),    // Text
                    base: Color::Rgb(36, 39, 58),       // Base
                },
            )
        }
    }

    pub fn dracula() -> Self {
        let cyan = Color::Rgb(139, 233, 253);
        Self {
            preview_title: cyan,
            list_date: cyan,
            popup_text: Color::Rgb(255, 85, 85), // Red (not Pink)
            icon_flutter: cyan,
            icon_go: cyan,
            icon_file: cyan,
            icon_maven: Color::Rgb(255, 85, 85), // Red (not Pink)
            icon_worktree_lock: Color::Rgb(248, 248, 242), // Foreground
            ..Self::from_palette(
                "Dracula",
                Some(Color::Rgb(40, 42, 54)),
                Palette {
                    accent1: Color::Rgb(189, 147, 249), // Purple
                    accent2: Color::Rgb(255, 121, 198), // Pink
                    warm: Color::Rgb(255, 184, 108),    // Orange
                    cool: Color::Rgb(139, 233, 253),    // Cyan
                    green: Color::Rgb(80, 250, 123),    // Green
                    yellow: Color::Rgb(241, 250, 140),  // Yellow
                    purple: Color::Rgb(189, 147, 249),  // Purple
                    overlay: Color::Rgb(98, 114, 164),  // Comment
                    subtext: Color::Rgb(139, 233, 253), // Cyan (for dates)
                    surface: Color::Rgb(68, 71, 90),    // Selection
                    text: Color::Rgb(248, 248, 242),    // Foreground
                    base: Color::Rgb(40, 42, 54),       // Background
                },
            )
        }
    }

    pub fn jetbrains_darcula() -> Self {
        Self {
            search_title: Color::Rgb(106, 135, 89),        // Green
            disk_title: Color::Rgb(204, 120, 50),          // Orange
            icon_maven: Color::Rgb(255, 198, 109),         // Gold
            icon_worktree: Color::Rgb(106, 135, 89),       // Green
            icon_worktree_lock: Color::Rgb(187, 187, 187), // Light Grey
            ..Self::from_palette(
                "JetBrains Darcula",
                Some(Color::Rgb(43, 43, 43)),
                Palette {
                    accent1: Color::Rgb(78, 124, 238),  // Blue
                    accent2: Color::Rgb(204, 120, 50),  // Orange
                    warm: Color::Rgb(204, 120, 50),     // Orange
                    cool: Color::Rgb(0, 173, 216),      // Go cyan
                    green: Color::Rgb(255, 198, 109),   // Gold
                    yellow: Color::Rgb(255, 198, 109),  // Gold
                    purple: Color::Rgb(152, 118, 170),  // Purple
                    overlay: Color::Rgb(128, 128, 128), // Grey
                    subtext: Color::Rgb(128, 128, 128), // Grey
                    surface: Color::Rgb(33, 66, 131),   // Selection
                    text: Color::Rgb(187, 187, 187),    // Light Grey
                    base: Color::Rgb(60, 63, 65),       // Bg
                },
            )
        }
    }

    pub fn gruvbox_dark() -> Self {
        Self {
            title_rs: Color::Rgb(250, 189, 47),            // Yellow
            search_title: Color::Rgb(184, 187, 38),        // Green
            folder_title: Color::Rgb(250, 189, 47),        // Yellow
            disk_title: Color::Rgb(254, 128, 25),          // Orange
            preview_title: Color::Rgb(131, 165, 152),      // Aqua
            status_message: Color::Rgb(215, 153, 33),      // Orange
            icon_flutter: Color::Rgb(131, 165, 152),       // Aqua
            icon_worktree_lock: Color::Rgb(168, 153, 132), // Grey (overlay)
            ..Self::from_palette(
                "Gruvbox Dark",
                Some(Color::Rgb(40, 40, 40)),
                Palette {
                    accent1: Color::Rgb(251, 73, 52),   // Red
                    accent2: Color::Rgb(251, 73, 52),   // Red
                    warm: Color::Rgb(254, 128, 25),     // Orange
                    cool: Color::Rgb(131, 165, 152),    // Aqua
                    green: Color::Rgb(184, 187, 38),    // Green
                    yellow: Color::Rgb(250, 189, 47),   // Yellow
                    purple: Color::Rgb(211, 134, 155),  // Purple
                    overlay: Color::Rgb(168, 153, 132), // Grey
                    subtext: Color::Rgb(146, 131, 116), // Grey
                    surface: Color::Rgb(80, 73, 69),    // Bg2
                    text: Color::Rgb(235, 219, 178),    // Fg
                    base: Color::Rgb(40, 40, 40),       // Bg0
                },
            )
        }
    }

    pub fn nord() -> Self {
        Self {
            search_title: Color::Rgb(163, 190, 140),  // Green (not warm)
            folder_title: Color::Rgb(235, 203, 139),  // Yellow (not green)
            disk_title: Color::Rgb(208, 135, 112),    // Aurora Orange
            icon_worktree: Color::Rgb(163, 190, 140), // Aurora Green
            ..Self::from_palette(
                "Nord",
                Some(Color::Rgb(46, 52, 64)),
                Palette {
                    accent1: Color::Rgb(136, 192, 208), // Frost Cyan
                    accent2: Color::Rgb(191, 97, 106),  // Aurora Red
                    warm: Color::Rgb(208, 135, 112),    // Aurora Orange
                    cool: Color::Rgb(136, 192, 208),    // Frost Cyan
                    green: Color::Rgb(163, 190, 140),   // Aurora Green
                    yellow: Color::Rgb(235, 203, 139),  // Aurora Yellow
                    purple: Color::Rgb(180, 142, 173),  // Aurora Purple
                    overlay: Color::Rgb(76, 86, 106),   // Polar Night 2
                    subtext: Color::Rgb(216, 222, 233), // Snow Storm
                    surface: Color::Rgb(67, 76, 94),    // Polar Night 3
                    text: Color::Rgb(236, 239, 244),    // Snow Storm 3
                    base: Color::Rgb(46, 52, 64),       // Polar Night 0
                },
            )
        }
    }

    pub fn tokyo_night() -> Self {
        let cyan = Color::Rgb(125, 207, 255);
        Self {
            search_title: Color::Rgb(158, 206, 106), // Green
            disk_title: Color::Rgb(255, 158, 100),   // Orange
            preview_title: cyan,
            icon_flutter: cyan,
            icon_go: cyan,
            icon_worktree: Color::Rgb(158, 206, 106), // Green
            ..Self::from_palette(
                "Tokyo Night",
                Some(Color::Rgb(26, 27, 38)),
                Palette {
                    accent1: Color::Rgb(122, 162, 247), // Blue
                    accent2: Color::Rgb(247, 118, 142), // Red
                    warm: Color::Rgb(255, 158, 100),    // Orange
                    cool: Color::Rgb(125, 207, 255),    // Cyan
                    green: Color::Rgb(224, 175, 104),   // Yellow (folder)
                    yellow: Color::Rgb(224, 175, 104),  // Yellow
                    purple: Color::Rgb(187, 154, 247),  // Purple
                    overlay: Color::Rgb(86, 95, 137),   // Comment
                    subtext: Color::Rgb(169, 177, 214), // Fg
                    surface: Color::Rgb(65, 72, 104),   // Terminal Black
                    text: Color::Rgb(192, 202, 245),    // Terminal White
                    base: Color::Rgb(26, 27, 38),       // Bg
                },
            )
        }
    }

    pub fn one_dark_pro() -> Self {
        let cyan = Color::Rgb(86, 182, 194);
        Self {
            preview_title: cyan,
            icon_flutter: cyan,
            icon_go: cyan,
            ..Self::from_palette(
                "One Dark Pro",
                Some(Color::Rgb(40, 44, 52)),
                Palette {
                    accent1: Color::Rgb(97, 175, 239),  // Blue
                    accent2: Color::Rgb(224, 108, 117), // Red
                    warm: Color::Rgb(209, 154, 102),    // Orange
                    cool: Color::Rgb(86, 182, 194),     // Cyan
                    green: Color::Rgb(152, 195, 121),   // Green
                    yellow: Color::Rgb(229, 192, 123),  // Yellow
                    purple: Color::Rgb(198, 120, 221),  // Purple
                    overlay: Color::Rgb(92, 99, 112),   // Comment
                    subtext: Color::Rgb(171, 178, 191), // Fg
                    surface: Color::Rgb(62, 68, 81),    // Selection
                    text: Color::Rgb(220, 223, 228),    // Bright Fg
                    base: Color::Rgb(40, 44, 52),       // Bg
                },
            )
        }
    }

    pub fn everforest() -> Self {
        Self::from_palette(
            "Everforest",
            Some(Color::Rgb(45, 51, 48)),
            Palette {
                accent1: Color::Rgb(127, 187, 179), // Aqua
                accent2: Color::Rgb(230, 126, 128), // Red
                warm: Color::Rgb(230, 152, 117),    // Orange
                cool: Color::Rgb(127, 187, 179),    // Aqua
                green: Color::Rgb(167, 192, 128),   // Green
                yellow: Color::Rgb(219, 188, 127),  // Yellow
                purple: Color::Rgb(214, 153, 182),  // Purple
                overlay: Color::Rgb(127, 132, 120), // Grey
                subtext: Color::Rgb(211, 198, 170), // Fg
                surface: Color::Rgb(80, 88, 77),    // Bg Visual
                text: Color::Rgb(211, 198, 170),    // Fg
                base: Color::Rgb(45, 51, 48),       // Bg
            },
        )
    }

    pub fn synthwave_84() -> Self {
        Self {
            search_title: Color::Rgb(255, 203, 107), // Yellow (not orange)
            legends_title: Color::Rgb(254, 78, 174), // Hot Pink
            popup_text: Color::Rgb(254, 78, 174),    // Hot Pink
            icon_rust: Color::Rgb(255, 140, 66),     // Orange (unique)
            icon_gitmodules: Color::Rgb(254, 78, 174), // Hot Pink
            ..Self::from_palette(
                "SynthWave '84",
                Some(Color::Rgb(38, 29, 53)),
                Palette {
                    accent1: Color::Rgb(54, 244, 244),  // Cyan
                    accent2: Color::Rgb(255, 126, 185), // Pink
                    warm: Color::Rgb(255, 140, 66),     // Orange
                    cool: Color::Rgb(54, 244, 244),     // Cyan
                    green: Color::Rgb(114, 241, 177),   // Green
                    yellow: Color::Rgb(255, 203, 107),  // Yellow
                    purple: Color::Rgb(254, 78, 174),   // Hot Pink
                    overlay: Color::Rgb(129, 91, 164),  // Purple dim
                    subtext: Color::Rgb(187, 186, 201), // Fg
                    surface: Color::Rgb(57, 43, 75),    // Selection
                    text: Color::Rgb(255, 255, 255),    // White
                    base: Color::Rgb(38, 29, 53),       // Bg
                },
            )
        }
    }

    pub fn oled_true_black() -> Self {
        Self {
            helpers_colors: Color::Rgb(100, 100, 100), // Grey (different from overlay)
            icon_rust: Color::Rgb(255, 120, 50),       // Bright Orange
            icon_mise: Color::Rgb(255, 180, 0),        // Orange (different from warm)
            ..Self::from_palette(
                "OLED True Black",
                Some(Color::Rgb(0, 0, 0)),
                Palette {
                    accent1: Color::Rgb(0, 200, 255),   // Bright Cyan
                    accent2: Color::Rgb(255, 80, 100),  // Bright Red
                    warm: Color::Rgb(255, 180, 0),      // Orange
                    cool: Color::Rgb(0, 200, 255),      // Bright Cyan
                    green: Color::Rgb(0, 230, 130),     // Bright Green
                    yellow: Color::Rgb(255, 220, 0),    // Yellow
                    purple: Color::Rgb(200, 100, 255),  // Purple
                    overlay: Color::Rgb(60, 60, 60),    // Dark Grey
                    subtext: Color::Rgb(180, 180, 180), // Light Grey
                    surface: Color::Rgb(30, 30, 30),    // Near Black
                    text: Color::Rgb(255, 255, 255),    // White
                    base: Color::Rgb(0, 0, 0),          // True Black
                },
            )
        }
    }

    pub fn silver_gray() -> Self {
        Self {
            preview_title: Color::Rgb(176, 196, 222), // Light Steel Blue
            icon_rust: Color::Rgb(210, 105, 30),      // Chocolate
            icon_go: Color::Rgb(176, 196, 222),       // Light Steel Blue
            ..Self::from_palette(
                "Silver Gray",
                Some(Color::Rgb(47, 47, 47)),
                Palette {
                    accent1: Color::Rgb(100, 149, 237), // Cornflower Blue
                    accent2: Color::Rgb(205, 92, 92),   // Indian Red
                    warm: Color::Rgb(218, 165, 32),     // Goldenrod
                    cool: Color::Rgb(176, 196, 222),    // Light Steel Blue
                    green: Color::Rgb(144, 238, 144),   // Light Green
                    yellow: Color::Rgb(240, 230, 140),  // Khaki
                    purple: Color::Rgb(186, 85, 211),   // Medium Orchid
                    overlay: Color::Rgb(128, 128, 128), // Gray
                    subtext: Color::Rgb(192, 192, 192), // Silver
                    surface: Color::Rgb(70, 70, 70),    // Dark Gray
                    text: Color::Rgb(245, 245, 245),    // White Smoke
                    base: Color::Rgb(47, 47, 47),       // Dark Bg
                },
            )
        }
    }

    pub fn black_and_white() -> Self {
        Self {
            icon_mise: Color::Gray,
            icon_gitmodules: Color::Gray,
            popup_text: Color::White,
            list_highlight_bg: Color::White,
            list_highlight_fg: Color::Gray,
            list_selected_fg: Color::Black,
            ..Self::from_palette(
                "Black & White",
                Some(Color::Black),
                Palette {
                    accent1: Color::White,
                    accent2: Color::White,
                    warm: Color::White,
                    cool: Color::White,
                    green: Color::White,
                    yellow: Color::White,
                    purple: Color::White,
                    overlay: Color::Gray,
                    subtext: Color::Gray,
                    surface: Color::White,
                    text: Color::White,
                    base: Color::Black,
                },
            )
        }
    }

    pub fn matrix() -> Self {
        let bright = Color::Rgb(0, 255, 65);
        let dark = Color::Rgb(0, 100, 30);
        let muted = Color::Rgb(0, 150, 40);
        let darker = Color::Rgb(0, 200, 50);
        Self {
            helpers_colors: Color::Rgb(0, 150, 40), // Muted green
            popup_text: Color::Rgb(0, 255, 65),     // Bright green
            icon_maven: Color::Rgb(0, 220, 55),
            icon_flutter: Color::Rgb(0, 200, 50), // Darker green
            icon_go: Color::Rgb(0, 180, 45),
            icon_mise: Color::Rgb(0, 150, 40), // Muted green
            icon_worktree: darker,
            icon_worktree_lock: Color::Rgb(0, 120, 35),
            icon_gitmodules: Color::Rgb(0, 180, 45),
            icon_folder: Color::Rgb(0, 220, 55),
            ..Self::from_palette(
                "Matrix",
                Some(Color::Rgb(0, 10, 0)),
                Palette {
                    accent1: bright,
                    accent2: darker,
                    warm: bright,
                    cool: Color::Rgb(0, 180, 45),
                    green: bright,
                    yellow: bright,
                    purple: darker,
                    overlay: dark,
                    subtext: muted,
                    surface: Color::Rgb(0, 80, 25),
                    text: bright,
                    base: Color::Rgb(0, 10, 0),
                },
            )
        }
    }

    pub fn tron() -> Self {
        let cyan = Color::Rgb(0, 255, 255);
        let orange = Color::Rgb(255, 150, 0);
        let dk_cyan = Color::Rgb(0, 150, 180);
        Self {
            search_title: cyan,
            folder_title: cyan,
            legends_title: Color::Rgb(0, 200, 220),
            popup_text: cyan,
            icon_maven: Color::Rgb(255, 100, 0),
            icon_go: Color::Rgb(0, 220, 230),
            icon_python: Color::Rgb(255, 200, 0),
            icon_worktree: cyan,
            icon_worktree_lock: dk_cyan,
            icon_gitmodules: Color::Rgb(0, 200, 220),
            icon_folder: cyan,
            ..Self::from_palette(
                "Tron",
                Some(Color::Rgb(0, 10, 15)),
                Palette {
                    accent1: cyan,
                    accent2: orange,
                    warm: orange,
                    cool: Color::Rgb(0, 220, 230),
                    green: cyan,
                    yellow: orange,
                    purple: Color::Rgb(0, 200, 220),
                    overlay: dk_cyan,
                    subtext: Color::Rgb(0, 180, 200),
                    surface: Color::Rgb(0, 80, 100),
                    text: cyan,
                    base: Color::Rgb(0, 10, 15),
                },
            )
        }
    }

    pub fn all() -> Vec<Theme> {
        vec![
            Theme::default_theme(),
            Theme::catppuccin_mocha(),
            Theme::catppuccin_macchiato(),
            Theme::dracula(),
            Theme::jetbrains_darcula(),
            Theme::gruvbox_dark(),
            Theme::nord(),
            Theme::tokyo_night(),
            Theme::one_dark_pro(),
            Theme::everforest(),
            Theme::synthwave_84(),
            Theme::oled_true_black(),
            Theme::silver_gray(),
            Theme::black_and_white(),
            Theme::matrix(),
            Theme::tron(),
        ]
    }
}
