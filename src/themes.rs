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
    pub list_match_fg: Color,
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
///   warm    → search_title, list_match_fg, icon_rust, icon_mise
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
            list_match_fg: p.warm,
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
            list_match_fg: Color::Rgb(106, 135, 89),       // Green
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
            list_match_fg: Color::Rgb(184, 187, 38),       // Green
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
            list_match_fg: Color::Rgb(163, 190, 140), // Green (not warm)
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
            search_title: Color::Rgb(158, 206, 106),  // Green
            list_match_fg: Color::Rgb(158, 206, 106), // Green
            disk_title: Color::Rgb(255, 158, 100),    // Orange
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
            list_match_fg: Color::Rgb(255, 203, 107), // Yellow (not orange)
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
            list_match_fg: cyan,
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

    pub fn monokai_pro() -> Self {
        Self {
            title_try: Color::Rgb(250, 250, 250),
            title_rs: Color::Rgb(250, 250, 250),
            search_title: Color::Rgb(252, 196, 7),
            list_match_fg: Color::Rgb(252, 196, 7),
            folder_title: Color::Rgb(166, 226, 45),
            disk_title: Color::Rgb(253, 151, 39),
            preview_title: Color::Rgb(102, 217, 240),
            legends_title: Color::Rgb(189, 147, 250),
            popup_text: Color::Rgb(249, 38, 114),
            icon_rust: Color::Rgb(230, 100, 50),
            icon_maven: Color::Rgb(255, 150, 50),
            icon_flutter: Color::Rgb(2, 123, 222),
            icon_go: Color::Rgb(0, 173, 216),
            icon_python: Color::Rgb(255, 220, 80),
            icon_mise: Color::Rgb(252, 196, 7),
            icon_worktree: Color::Rgb(166, 226, 45),
            icon_worktree_lock: Color::Rgb(166, 173, 200),
            icon_gitmodules: Color::Rgb(189, 147, 250),
            icon_git: Color::Rgb(249, 38, 114),
            icon_folder: Color::Rgb(166, 226, 45),
            icon_file: Color::Rgb(166, 173, 200),
            name: "Monokai Pro".to_string(),
            background: Some(Color::Rgb(39, 40, 34)),
            search_border: Color::Rgb(74, 73, 68),
            folder_border: Color::Rgb(74, 73, 68),
            disk_border: Color::Rgb(74, 73, 68),
            preview_border: Color::Rgb(74, 73, 68),
            legends_border: Color::Rgb(74, 73, 68),
            list_date: Color::Rgb(166, 173, 200),
            list_highlight_bg: Color::Rgb(60, 58, 50),
            list_highlight_fg: Color::Rgb(250, 250, 250),
            list_selected_fg: Color::Rgb(250, 250, 250),
            helpers_colors: Color::Rgb(117, 113, 106),
            status_message: Color::Rgb(253, 151, 39),
            popup_bg: Color::Rgb(50, 50, 42),
        }
    }

    pub fn solarized_dark() -> Self {
        Self {
            search_title: Color::Rgb(133, 153, 0),
            list_match_fg: Color::Rgb(133, 153, 0),
            folder_title: Color::Rgb(181, 137, 0),
            disk_title: Color::Rgb(203, 75, 22),
            preview_title: Color::Rgb(38, 139, 210),
            legends_title: Color::Rgb(108, 113, 196),
            status_message: Color::Rgb(203, 75, 22),
            icon_rust: Color::Rgb(203, 75, 22),
            icon_maven: Color::Rgb(211, 54, 130),
            icon_flutter: Color::Rgb(38, 139, 210),
            icon_go: Color::Rgb(44, 160, 160),
            icon_python: Color::Rgb(181, 137, 0),
            icon_mise: Color::Rgb(133, 153, 0),
            icon_worktree: Color::Rgb(133, 153, 0),
            icon_worktree_lock: Color::Rgb(101, 123, 131),
            icon_gitmodules: Color::Rgb(108, 113, 196),
            icon_git: Color::Rgb(211, 54, 130),
            icon_folder: Color::Rgb(181, 137, 0),
            icon_file: Color::Rgb(101, 123, 131),
            ..Self::from_palette(
                "Solarized Dark",
                Some(Color::Rgb(0, 43, 54)),
                Palette {
                    accent1: Color::Rgb(38, 139, 210),  // Blue
                    accent2: Color::Rgb(211, 54, 130),  // Magenta
                    warm: Color::Rgb(203, 75, 22),      // Orange
                    cool: Color::Rgb(44, 160, 160),     // Cyan
                    green: Color::Rgb(133, 153, 0),     // Green
                    yellow: Color::Rgb(181, 137, 0),    // Yellow
                    purple: Color::Rgb(108, 113, 196),  // Violet
                    overlay: Color::Rgb(88, 110, 117),  // Base01
                    subtext: Color::Rgb(101, 123, 131), // Base00
                    surface: Color::Rgb(7, 54, 66),     // Base02
                    text: Color::Rgb(238, 232, 213),    // Base2
                    base: Color::Rgb(0, 43, 54),        // Base03
                },
            )
        }
    }

    pub fn night_owl() -> Self {
        let cyan = Color::Rgb(136, 192, 208);
        Self {
            search_title: Color::Rgb(187, 205, 136),
            list_match_fg: Color::Rgb(187, 205, 136),
            folder_title: Color::Rgb(229, 192, 123),
            disk_title: Color::Rgb(209, 154, 102),
            preview_title: cyan,
            legends_title: Color::Rgb(198, 120, 221),
            popup_text: Color::Rgb(255, 117, 107),
            icon_rust: Color::Rgb(255, 117, 107),
            icon_maven: Color::Rgb(255, 117, 107),
            icon_flutter: cyan,
            icon_go: cyan,
            icon_python: Color::Rgb(229, 192, 123),
            icon_mise: Color::Rgb(187, 205, 136),
            icon_worktree: Color::Rgb(187, 205, 136),
            icon_worktree_lock: Color::Rgb(130, 145, 162),
            icon_gitmodules: Color::Rgb(198, 120, 221),
            icon_git: Color::Rgb(255, 117, 107),
            icon_folder: Color::Rgb(229, 192, 123),
            icon_file: Color::Rgb(130, 145, 162),
            ..Self::from_palette(
                "Night Owl",
                Some(Color::Rgb(10, 14, 30)),
                Palette {
                    accent1: Color::Rgb(97, 175, 239),  // Blue
                    accent2: Color::Rgb(255, 117, 107), // Red
                    warm: Color::Rgb(209, 154, 102),    // Orange
                    cool: cyan,
                    green: Color::Rgb(187, 205, 136),   // Green
                    yellow: Color::Rgb(229, 192, 123),  // Yellow
                    purple: Color::Rgb(198, 120, 221),  // Purple
                    overlay: Color::Rgb(85, 104, 130),  // Comment
                    subtext: Color::Rgb(130, 145, 162), // Foreground
                    surface: Color::Rgb(40, 50, 80),    // Selection
                    text: Color::Rgb(197, 214, 224),    // Foreground
                    base: Color::Rgb(10, 14, 30),       // Background
                },
            )
        }
    }

    pub fn gruvbox_material() -> Self {
        Self {
            title_rs: Color::Rgb(251, 191, 36),
            search_title: Color::Rgb(166, 227, 161),
            list_match_fg: Color::Rgb(166, 227, 161),
            folder_title: Color::Rgb(251, 191, 36),
            disk_title: Color::Rgb(254, 128, 25),
            preview_title: Color::Rgb(131, 165, 152),
            legends_title: Color::Rgb(211, 134, 155),
            status_message: Color::Rgb(254, 128, 25),
            popup_text: Color::Rgb(211, 134, 155),
            icon_rust: Color::Rgb(251, 73, 52),
            icon_flutter: Color::Rgb(131, 165, 152),
            icon_go: Color::Rgb(131, 165, 152),
            icon_worktree: Color::Rgb(166, 227, 161),
            icon_worktree_lock: Color::Rgb(168, 153, 132),
            icon_gitmodules: Color::Rgb(211, 134, 155),
            icon_folder: Color::Rgb(251, 191, 36),
            ..Self::from_palette(
                "Gruvbox Material",
                Some(Color::Rgb(45, 40, 36)),
                Palette {
                    accent1: Color::Rgb(251, 73, 52),   // Red
                    accent2: Color::Rgb(251, 191, 36),  // Yellow
                    warm: Color::Rgb(254, 128, 25),     // Orange
                    cool: Color::Rgb(131, 165, 152),    // Aqua
                    green: Color::Rgb(166, 227, 161),   // Green
                    yellow: Color::Rgb(251, 191, 36),   // Yellow
                    purple: Color::Rgb(211, 134, 155),  // Purple
                    overlay: Color::Rgb(168, 153, 132), // Grey
                    subtext: Color::Rgb(146, 131, 116), // Grey
                    surface: Color::Rgb(60, 54, 48),    // Bg2
                    text: Color::Rgb(235, 219, 178),    // Fg
                    base: Color::Rgb(45, 40, 36),       // Bg0
                },
            )
        }
    }

    pub fn zenburn() -> Self {
        Self {
            search_title: Color::Rgb(104, 151, 71),
            list_match_fg: Color::Rgb(104, 151, 71),
            folder_title: Color::Rgb(204, 145, 91),
            disk_title: Color::Rgb(214, 114, 91),
            preview_title: Color::Rgb(70, 130, 180),
            legends_title: Color::Rgb(137, 123, 152),
            status_message: Color::Rgb(214, 114, 91),
            popup_text: Color::Rgb(214, 114, 91),
            icon_rust: Color::Rgb(214, 114, 91),
            icon_maven: Color::Rgb(214, 114, 91),
            icon_flutter: Color::Rgb(70, 130, 180),
            icon_go: Color::Rgb(70, 130, 180),
            icon_python: Color::Rgb(204, 145, 91),
            icon_mise: Color::Rgb(104, 151, 71),
            icon_worktree: Color::Rgb(104, 151, 71),
            icon_worktree_lock: Color::Rgb(136, 136, 136),
            icon_gitmodules: Color::Rgb(137, 123, 152),
            icon_git: Color::Rgb(214, 114, 91),
            icon_folder: Color::Rgb(204, 145, 91),
            icon_file: Color::Rgb(136, 136, 136),
            ..Self::from_palette(
                "Zenburn",
                Some(Color::Rgb(48, 48, 48)),
                Palette {
                    accent1: Color::Rgb(70, 130, 180),  // Steel Blue
                    accent2: Color::Rgb(214, 114, 91),  // Red
                    warm: Color::Rgb(204, 145, 91),     // Orange
                    cool: Color::Rgb(70, 130, 180),     // Steel Blue
                    green: Color::Rgb(104, 151, 71),    // Green
                    yellow: Color::Rgb(204, 145, 91),   // Orange
                    purple: Color::Rgb(137, 123, 152),  // Purple
                    overlay: Color::Rgb(100, 100, 100), // Grey
                    subtext: Color::Rgb(136, 136, 136), // Grey
                    surface: Color::Rgb(60, 60, 60),    // Bg2
                    text: Color::Rgb(192, 192, 192),    // Fg
                    base: Color::Rgb(48, 48, 48),       // Bg
                },
            )
        }
    }

    pub fn solarized_light() -> Self {
        Self {
            search_title: Color::Rgb(133, 153, 0),
            list_match_fg: Color::Rgb(133, 153, 0),
            folder_title: Color::Rgb(181, 137, 0),
            disk_title: Color::Rgb(203, 75, 22),
            preview_title: Color::Rgb(38, 139, 210),
            legends_title: Color::Rgb(108, 113, 196),
            status_message: Color::Rgb(203, 75, 22),
            icon_rust: Color::Rgb(203, 75, 22),
            icon_maven: Color::Rgb(211, 54, 130),
            icon_flutter: Color::Rgb(38, 139, 210),
            icon_go: Color::Rgb(44, 160, 160),
            icon_python: Color::Rgb(181, 137, 0),
            icon_mise: Color::Rgb(133, 153, 0),
            icon_worktree: Color::Rgb(133, 153, 0),
            icon_worktree_lock: Color::Rgb(101, 123, 131),
            icon_gitmodules: Color::Rgb(108, 113, 196),
            icon_git: Color::Rgb(211, 54, 130),
            icon_folder: Color::Rgb(181, 137, 0),
            icon_file: Color::Rgb(101, 123, 131),
            ..Self::from_palette(
                "Solarized Light",
                Some(Color::Rgb(253, 246, 227)),
                Palette {
                    accent1: Color::Rgb(38, 139, 210),  // Blue
                    accent2: Color::Rgb(211, 54, 130),  // Magenta
                    warm: Color::Rgb(203, 75, 22),      // Orange
                    cool: Color::Rgb(44, 160, 160),     // Cyan
                    green: Color::Rgb(133, 153, 0),     // Green
                    yellow: Color::Rgb(181, 137, 0),    // Yellow
                    purple: Color::Rgb(108, 113, 196),  // Violet
                    overlay: Color::Rgb(147, 147, 147), // Base1
                    subtext: Color::Rgb(101, 123, 131), // Base00
                    surface: Color::Rgb(238, 232, 213), // Base2
                    text: Color::Rgb(7, 54, 66),        // Base03
                    base: Color::Rgb(253, 246, 227),    // Base3
                },
            )
        }
    }

    pub fn monokai_pro_light() -> Self {
        Self {
            title_try: Color::Rgb(50, 50, 40),
            title_rs: Color::Rgb(50, 50, 40),
            search_title: Color::Rgb(180, 120, 10),
            list_match_fg: Color::Rgb(180, 120, 10),
            folder_title: Color::Rgb(100, 150, 30),
            disk_title: Color::Rgb(200, 100, 20),
            preview_title: Color::Rgb(20, 120, 180),
            legends_title: Color::Rgb(130, 90, 180),
            popup_text: Color::Rgb(180, 20, 80),
            icon_rust: Color::Rgb(200, 80, 40),
            icon_maven: Color::Rgb(200, 120, 40),
            icon_flutter: Color::Rgb(2, 100, 180),
            icon_go: Color::Rgb(0, 140, 180),
            icon_python: Color::Rgb(180, 160, 60),
            icon_mise: Color::Rgb(180, 120, 10),
            icon_worktree: Color::Rgb(100, 150, 30),
            icon_worktree_lock: Color::Rgb(100, 100, 90),
            icon_gitmodules: Color::Rgb(130, 90, 180),
            icon_git: Color::Rgb(180, 20, 80),
            icon_folder: Color::Rgb(100, 150, 30),
            icon_file: Color::Rgb(100, 100, 90),
            name: "Monokai Pro Light".to_string(),
            background: Some(Color::Rgb(250, 250, 240)),
            search_border: Color::Rgb(180, 175, 165),
            folder_border: Color::Rgb(180, 175, 165),
            disk_border: Color::Rgb(180, 175, 165),
            preview_border: Color::Rgb(180, 175, 165),
            legends_border: Color::Rgb(180, 175, 165),
            list_date: Color::Rgb(100, 100, 90),
            list_highlight_bg: Color::Rgb(230, 225, 210),
            list_highlight_fg: Color::Rgb(50, 50, 40),
            list_selected_fg: Color::Rgb(50, 50, 40),
            helpers_colors: Color::Rgb(140, 140, 130),
            status_message: Color::Rgb(200, 100, 20),
            popup_bg: Color::Rgb(245, 245, 235),
        }
    }

    pub fn light_owl() -> Self {
        let cyan = Color::Rgb(70, 150, 180);
        Self {
            search_title: Color::Rgb(120, 140, 60),
            list_match_fg: Color::Rgb(120, 140, 60),
            folder_title: Color::Rgb(170, 150, 80),
            disk_title: Color::Rgb(180, 110, 70),
            preview_title: cyan,
            legends_title: Color::Rgb(140, 80, 170),
            popup_text: Color::Rgb(200, 60, 50),
            icon_rust: Color::Rgb(200, 60, 50),
            icon_maven: Color::Rgb(200, 60, 50),
            icon_flutter: cyan,
            icon_go: cyan,
            icon_python: Color::Rgb(170, 150, 80),
            icon_mise: Color::Rgb(120, 140, 60),
            icon_worktree: Color::Rgb(120, 140, 60),
            icon_worktree_lock: Color::Rgb(100, 110, 120),
            icon_gitmodules: Color::Rgb(140, 80, 170),
            icon_git: Color::Rgb(200, 60, 50),
            icon_folder: Color::Rgb(170, 150, 80),
            icon_file: Color::Rgb(100, 110, 120),
            ..Self::from_palette(
                "Light Owl",
                Some(Color::Rgb(250, 250, 245)),
                Palette {
                    accent1: Color::Rgb(60, 130, 200), // Blue
                    accent2: Color::Rgb(200, 60, 50),  // Red
                    warm: Color::Rgb(180, 110, 70),    // Orange
                    cool: cyan,
                    green: Color::Rgb(120, 140, 60),    // Green
                    yellow: Color::Rgb(170, 150, 80),   // Yellow
                    purple: Color::Rgb(140, 80, 170),   // Purple
                    overlay: Color::Rgb(160, 170, 180), // Comment
                    subtext: Color::Rgb(100, 110, 120), // Foreground
                    surface: Color::Rgb(220, 225, 235), // Selection
                    text: Color::Rgb(40, 50, 60),       // Foreground
                    base: Color::Rgb(250, 250, 245),    // Background
                },
            )
        }
    }

    pub fn cyberpunk() -> Self {
        let neon_pink = Color::Rgb(255, 0, 128);
        let neon_blue = Color::Rgb(0, 255, 255);
        let neon_purple = Color::Rgb(191, 0, 255);
        let neon_yellow = Color::Rgb(255, 255, 0);
        Self {
            title_try: neon_blue,
            title_rs: neon_pink,
            search_title: neon_yellow,
            list_match_fg: neon_yellow,
            folder_title: neon_purple,
            disk_title: neon_pink,
            preview_title: neon_blue,
            legends_title: neon_purple,
            status_message: neon_pink,
            popup_text: neon_pink,
            icon_rust: neon_pink,
            icon_maven: neon_blue,
            icon_flutter: neon_blue,
            icon_go: neon_blue,
            icon_python: neon_yellow,
            icon_mise: neon_yellow,
            icon_worktree: neon_purple,
            icon_worktree_lock: Color::Rgb(128, 128, 128),
            icon_gitmodules: neon_purple,
            icon_git: neon_pink,
            icon_folder: neon_purple,
            icon_file: Color::Rgb(180, 180, 180),
            ..Self::from_palette(
                "Cyberpunk",
                Some(Color::Rgb(15, 15, 25)),
                Palette {
                    accent1: neon_blue,
                    accent2: neon_pink,
                    warm: neon_yellow,
                    cool: neon_blue,
                    green: Color::Rgb(0, 255, 128),
                    yellow: neon_yellow,
                    purple: neon_purple,
                    overlay: Color::Rgb(70, 70, 90),
                    subtext: Color::Rgb(150, 150, 170),
                    surface: Color::Rgb(40, 40, 60),
                    text: Color::Rgb(240, 240, 245),
                    base: Color::Rgb(15, 15, 25),
                },
            )
        }
    }

    pub fn paper() -> Self {
        Self {
            title_try: Color::Rgb(0, 0, 0),
            title_rs: Color::Rgb(0, 0, 0),
            search_title: Color::Rgb(70, 100, 160),
            list_match_fg: Color::Rgb(70, 100, 160),
            folder_title: Color::Rgb(50, 100, 50),
            disk_title: Color::Rgb(150, 100, 50),
            preview_title: Color::Rgb(80, 80, 80),
            legends_title: Color::Rgb(100, 50, 100),
            popup_text: Color::Rgb(0, 0, 0),
            icon_rust: Color::Rgb(150, 50, 50),
            icon_maven: Color::Rgb(100, 50, 50),
            icon_flutter: Color::Rgb(50, 80, 150),
            icon_go: Color::Rgb(50, 100, 120),
            icon_python: Color::Rgb(50, 80, 120),
            icon_mise: Color::Rgb(70, 100, 160),
            icon_worktree: Color::Rgb(50, 100, 50),
            icon_worktree_lock: Color::Rgb(120, 120, 120),
            icon_gitmodules: Color::Rgb(100, 50, 100),
            icon_git: Color::Rgb(0, 0, 0),
            icon_folder: Color::Rgb(50, 100, 50),
            icon_file: Color::Rgb(80, 80, 80),
            ..Self::from_palette(
                "Paper",
                Some(Color::Rgb(250, 248, 245)),
                Palette {
                    accent1: Color::Rgb(50, 80, 150),   // Blue
                    accent2: Color::Rgb(0, 0, 0),       // Black
                    warm: Color::Rgb(150, 100, 50),     // Brown
                    cool: Color::Rgb(50, 100, 120),     // Teal
                    green: Color::Rgb(50, 100, 50),     // Green
                    yellow: Color::Rgb(150, 100, 50),   // Brown
                    purple: Color::Rgb(100, 50, 100),   // Purple
                    overlay: Color::Rgb(180, 180, 180), // Grey
                    subtext: Color::Rgb(80, 80, 80),    // Grey
                    surface: Color::Rgb(235, 230, 220), // Cream
                    text: Color::Rgb(30, 30, 30),       // Dark
                    base: Color::Rgb(250, 248, 245),    // Paper
                },
            )
        }
    }

    pub fn hacker() -> Self {
        let bright = Color::Rgb(0, 255, 65);
        let dark = Color::Rgb(0, 80, 20);
        Self {
            title_try: bright,
            title_rs: bright,
            search_title: bright,
            list_match_fg: bright,
            folder_title: bright,
            disk_title: bright,
            preview_title: bright,
            legends_title: bright,
            status_message: bright,
            popup_text: bright,
            icon_rust: bright,
            icon_maven: bright,
            icon_flutter: bright,
            icon_go: bright,
            icon_python: bright,
            icon_mise: bright,
            icon_worktree: bright,
            icon_worktree_lock: Color::Rgb(0, 150, 40),
            icon_gitmodules: bright,
            icon_git: bright,
            icon_folder: bright,
            icon_file: bright,
            ..Self::from_palette(
                "Hacker",
                Some(Color::Rgb(0, 10, 0)),
                Palette {
                    accent1: bright,
                    accent2: bright,
                    warm: bright,
                    cool: bright,
                    green: bright,
                    yellow: bright,
                    purple: bright,
                    overlay: dark,
                    subtext: Color::Rgb(0, 150, 40),
                    surface: Color::Rgb(0, 40, 10),
                    text: bright,
                    base: Color::Rgb(0, 10, 0),
                },
            )
        }
    }

    pub fn ubuntu() -> Self {
        Self {
            search_title: Color::Rgb(166, 82, 33),
            list_match_fg: Color::Rgb(166, 82, 33),
            folder_title: Color::Rgb(165, 79, 30),
            disk_title: Color::Rgb(206, 92, 30),
            preview_title: Color::Rgb(84, 84, 84),
            legends_title: Color::Rgb(136, 36, 35),
            status_message: Color::Rgb(206, 92, 30),
            popup_text: Color::Rgb(136, 36, 35),
            icon_rust: Color::Rgb(206, 92, 30),
            icon_maven: Color::Rgb(165, 79, 30),
            icon_flutter: Color::Rgb(84, 84, 84),
            icon_go: Color::Rgb(84, 84, 84),
            icon_python: Color::Rgb(165, 79, 30),
            icon_mise: Color::Rgb(166, 82, 33),
            icon_worktree: Color::Rgb(166, 82, 33),
            icon_worktree_lock: Color::Rgb(117, 117, 117),
            icon_gitmodules: Color::Rgb(136, 36, 35),
            icon_git: Color::Rgb(136, 36, 35),
            icon_folder: Color::Rgb(165, 79, 30),
            icon_file: Color::Rgb(117, 117, 117),
            ..Self::from_palette(
                "Ubuntu",
                Some(Color::Rgb(48, 42, 42)),
                Palette {
                    accent1: Color::Rgb(84, 84, 84),    // Dark grey
                    accent2: Color::Rgb(136, 36, 35),   // Dark red
                    warm: Color::Rgb(166, 82, 33),      // Orange
                    cool: Color::Rgb(84, 84, 84),       // Grey
                    green: Color::Rgb(166, 82, 33),     // Orange
                    yellow: Color::Rgb(165, 79, 30),    // Orange
                    purple: Color::Rgb(136, 36, 35),    // Dark red
                    overlay: Color::Rgb(117, 117, 117), // Grey
                    subtext: Color::Rgb(117, 117, 117), // Grey
                    surface: Color::Rgb(68, 55, 55),    // Dark brown
                    text: Color::Rgb(250, 250, 250),    // White
                    base: Color::Rgb(48, 42, 42),       // Dark brown
                },
            )
        }
    }

    pub fn man_page() -> Self {
        Self {
            title_try: Color::Rgb(0, 0, 0),
            title_rs: Color::Rgb(0, 0, 0),
            search_title: Color::Rgb(34, 139, 34),
            list_match_fg: Color::Rgb(34, 139, 34),
            folder_title: Color::Rgb(0, 0, 0),
            disk_title: Color::Rgb(178, 34, 34),
            preview_title: Color::Rgb(0, 0, 139),
            legends_title: Color::Rgb(128, 0, 128),
            status_message: Color::Rgb(178, 34, 34),
            popup_text: Color::Rgb(0, 0, 0),
            icon_rust: Color::Rgb(178, 34, 34),
            icon_maven: Color::Rgb(0, 0, 0),
            icon_flutter: Color::Rgb(0, 0, 139),
            icon_go: Color::Rgb(0, 139, 139),
            icon_python: Color::Rgb(0, 0, 0),
            icon_mise: Color::Rgb(34, 139, 34),
            icon_worktree: Color::Rgb(34, 139, 34),
            icon_worktree_lock: Color::Rgb(100, 100, 100),
            icon_gitmodules: Color::Rgb(128, 0, 128),
            icon_git: Color::Rgb(0, 0, 0),
            icon_folder: Color::Rgb(0, 0, 0),
            icon_file: Color::Rgb(100, 100, 100),
            ..Self::from_palette(
                "Man Page",
                Some(Color::Rgb(250, 250, 250)),
                Palette {
                    accent1: Color::Rgb(0, 0, 139),     // Dark Blue
                    accent2: Color::Rgb(0, 0, 0),       // Black
                    warm: Color::Rgb(178, 34, 34),      // Firebrick
                    cool: Color::Rgb(0, 139, 139),      // Dark Cyan
                    green: Color::Rgb(34, 139, 34),     // Forest Green
                    yellow: Color::Rgb(0, 0, 0),        // Black
                    purple: Color::Rgb(128, 0, 128),    // Purple
                    overlay: Color::Rgb(150, 150, 150), // Grey
                    subtext: Color::Rgb(100, 100, 100), // Grey
                    surface: Color::Rgb(230, 230, 230), // Light Grey
                    text: Color::Rgb(0, 0, 0),          // Black
                    base: Color::Rgb(250, 250, 250),    // White
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
            Theme::monokai_pro(),
            Theme::solarized_dark(),
            Theme::night_owl(),
            Theme::gruvbox_material(),
            Theme::zenburn(),
            Theme::solarized_light(),
            Theme::monokai_pro_light(),
            Theme::light_owl(),
            Theme::cyberpunk(),
            Theme::paper(),
            Theme::hacker(),
            Theme::ubuntu(),
            Theme::man_page(),
        ]
    }
}
