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

impl Theme {
    /// Default theme with the original hardcoded colors
    pub fn default_theme() -> Self {
        Self {
            name: "Default".to_string(),
            background: None, // Transparent by default
            title_try: Color::Rgb(137, 180, 250),         // Blue
            title_rs: Color::Rgb(243, 139, 168),          // Red
            search_title: Color::Rgb(250, 179, 135),      // Peach
            search_border: Color::Rgb(147, 153, 178),     // Overlay
            folder_title: Color::Rgb(166, 227, 161),      // Green
            folder_border: Color::Rgb(147, 153, 178),     // Overlay
            disk_title: Color::Rgb(249, 226, 175),        // Yellow
            disk_border: Color::Rgb(147, 153, 178),       // Overlay
            preview_title: Color::Rgb(137, 180, 250),     // Blue
            preview_border: Color::Rgb(147, 153, 178),    // Overlay
            legends_title: Color::Rgb(203, 166, 247),     // Mauve
            legends_border: Color::Rgb(147, 153, 178),    // Overlay
            list_date: Color::Rgb(166, 173, 200),         // Subtext
            list_highlight_bg: Color::Rgb(88, 91, 112),   // Surface
            list_highlight_fg: Color::Rgb(205, 214, 244), // Text
            helpers_colors: Color::Rgb(147, 153, 178),    // Overlay
            status_message: Color::Rgb(249, 226, 175),    // Yellow
            popup_bg: Color::Rgb(30, 30, 46),             // Base
            popup_text: Color::Rgb(243, 139, 168),        // Red
            // Default icon colors (original hardcoded values)
            icon_rust: Color::Rgb(230, 100, 50),   // Rust orange
            icon_maven: Color::Rgb(255, 150, 50),  // Maven orange
            icon_flutter: Color::Rgb(2, 123, 222), // Flutter blue
            icon_go: Color::Rgb(0, 173, 216),      // Go cyan
            icon_python: Color::Yellow,            // Python yellow
            icon_mise: Color::Rgb(250, 179, 135),  // Mise peach
            icon_worktree: Color::Rgb(100, 180, 100), // Worktree green
            icon_worktree_lock: Color::White,      // Lock white
            icon_gitmodules: Color::Rgb(180, 130, 200), // Submodules purple
            icon_git: Color::Rgb(240, 80, 50),     // Git red-orange
            icon_folder: Color::Rgb(249, 226, 175), // Folder yellow
            icon_file: Color::Rgb(166, 173, 200),  // File grey
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "Catppuccin Mocha".to_string(),
            background: Some(Color::Rgb(30, 30, 46)), // Base
            title_try: Color::Rgb(137, 180, 250),         // Blue
            title_rs: Color::Rgb(243, 139, 168),          // Red
            search_title: Color::Rgb(250, 179, 135),      // Peach
            search_border: Color::Rgb(147, 153, 178),     // Overlay2
            folder_title: Color::Rgb(166, 227, 161),      // Green
            folder_border: Color::Rgb(147, 153, 178),     // Overlay2
            disk_title: Color::Rgb(249, 226, 175),        // Yellow
            disk_border: Color::Rgb(147, 153, 178),       // Overlay2
            preview_title: Color::Rgb(137, 180, 250),     // Blue
            preview_border: Color::Rgb(147, 153, 178),    // Overlay2
            legends_title: Color::Rgb(203, 166, 247),     // Mauve
            legends_border: Color::Rgb(147, 153, 178),    // Overlay2
            list_date: Color::Rgb(166, 173, 200),         // Subtext0
            list_highlight_bg: Color::Rgb(88, 91, 112),   // Surface2
            list_highlight_fg: Color::Rgb(205, 214, 244), // Text
            helpers_colors: Color::Rgb(147, 153, 178),    // Overlay2
            status_message: Color::Rgb(249, 226, 175),    // Yellow
            popup_bg: Color::Rgb(30, 30, 46),             // Base
            popup_text: Color::Rgb(243, 139, 168),        // Red
            // Catppuccin icon colors
            icon_rust: Color::Rgb(250, 179, 135),     // Peach
            icon_maven: Color::Rgb(243, 139, 168),    // Red
            icon_flutter: Color::Rgb(137, 180, 250),  // Blue
            icon_go: Color::Rgb(148, 226, 213),       // Teal
            icon_python: Color::Rgb(249, 226, 175),   // Yellow
            icon_mise: Color::Rgb(250, 179, 135),     // Peach
            icon_worktree: Color::Rgb(166, 227, 161), // Green
            icon_worktree_lock: Color::Rgb(166, 173, 200), // Subtext0
            icon_gitmodules: Color::Rgb(203, 166, 247), // Mauve
            icon_git: Color::Rgb(243, 139, 168),      // Red
            icon_folder: Color::Rgb(249, 226, 175),   // Yellow
            icon_file: Color::Rgb(166, 173, 200),     // Subtext0
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "Dracula".to_string(),
            background: Some(Color::Rgb(40, 42, 54)), // Background
            title_try: Color::Rgb(189, 147, 249),      // Purple
            title_rs: Color::Rgb(255, 121, 198),       // Pink
            search_title: Color::Rgb(255, 184, 108),   // Orange
            search_border: Color::Rgb(98, 114, 164),   // Comment
            folder_title: Color::Rgb(80, 250, 123),    // Green
            folder_border: Color::Rgb(98, 114, 164),   // Comment
            disk_title: Color::Rgb(241, 250, 140),     // Yellow
            disk_border: Color::Rgb(98, 114, 164),     // Comment
            preview_title: Color::Rgb(139, 233, 253),  // Cyan
            preview_border: Color::Rgb(98, 114, 164),  // Comment
            legends_title: Color::Rgb(189, 147, 249),  // Purple
            legends_border: Color::Rgb(98, 114, 164),  // Comment
            list_date: Color::Rgb(139, 233, 253),      // Cyan
            list_highlight_bg: Color::Rgb(68, 71, 90), // Selection
            list_highlight_fg: Color::Rgb(248, 248, 242), // Foreground
            helpers_colors: Color::Rgb(98, 114, 164),  // Comment
            status_message: Color::Rgb(241, 250, 140), // Yellow
            popup_bg: Color::Rgb(40, 42, 54),          // Background
            popup_text: Color::Rgb(255, 85, 85),       // Red
            // Dracula icon colors
            icon_rust: Color::Rgb(255, 184, 108),    // Orange
            icon_maven: Color::Rgb(255, 85, 85),     // Red
            icon_flutter: Color::Rgb(139, 233, 253), // Cyan
            icon_go: Color::Rgb(139, 233, 253),      // Cyan
            icon_python: Color::Rgb(241, 250, 140),  // Yellow
            icon_mise: Color::Rgb(255, 184, 108),    // Orange
            icon_worktree: Color::Rgb(80, 250, 123), // Green
            icon_worktree_lock: Color::Rgb(248, 248, 242), // Foreground
            icon_gitmodules: Color::Rgb(189, 147, 249), // Purple
            icon_git: Color::Rgb(255, 121, 198),     // Pink
            icon_folder: Color::Rgb(241, 250, 140),  // Yellow
            icon_file: Color::Rgb(139, 233, 253),    // Cyan
        }
    }

    pub fn jetbrains_darcula() -> Self {
        Self {
            name: "JetBrains Darcula".to_string(),
            background: Some(Color::Rgb(43, 43, 43)), // Bg
            title_try: Color::Rgb(78, 124, 238),        // Blueish
            title_rs: Color::Rgb(204, 120, 50),         // Orange
            search_title: Color::Rgb(106, 135, 89),     // Green
            search_border: Color::Rgb(128, 128, 128),   // Grey
            folder_title: Color::Rgb(255, 198, 109),    // Gold
            folder_border: Color::Rgb(128, 128, 128),   // Grey
            disk_title: Color::Rgb(204, 120, 50),       // Orange
            disk_border: Color::Rgb(128, 128, 128),     // Grey
            preview_title: Color::Rgb(78, 124, 238),    // Blueish
            preview_border: Color::Rgb(128, 128, 128),  // Grey
            legends_title: Color::Rgb(152, 118, 170),   // Purple
            legends_border: Color::Rgb(128, 128, 128),  // Grey
            list_date: Color::Rgb(128, 128, 128),       // Grey
            list_highlight_bg: Color::Rgb(33, 66, 131), // Selection
            list_highlight_fg: Color::Rgb(187, 187, 187), // Light Grey
            helpers_colors: Color::Rgb(128, 128, 128),  // Grey
            status_message: Color::Rgb(255, 198, 109),  // Gold
            popup_bg: Color::Rgb(60, 63, 65),           // Bg
            popup_text: Color::Rgb(204, 120, 50),       // Orange
            // JetBrains Darcula icon colors
            icon_rust: Color::Rgb(204, 120, 50),     // Orange
            icon_maven: Color::Rgb(255, 198, 109),   // Gold
            icon_flutter: Color::Rgb(78, 124, 238),  // Blue
            icon_go: Color::Rgb(0, 173, 216),        // Go cyan
            icon_python: Color::Rgb(255, 198, 109),  // Gold
            icon_mise: Color::Rgb(204, 120, 50),     // Orange
            icon_worktree: Color::Rgb(106, 135, 89), // Green
            icon_worktree_lock: Color::Rgb(187, 187, 187), // Light Grey
            icon_gitmodules: Color::Rgb(152, 118, 170), // Purple
            icon_git: Color::Rgb(204, 120, 50),      // Orange
            icon_folder: Color::Rgb(255, 198, 109),  // Gold
            icon_file: Color::Rgb(128, 128, 128),    // Grey
        }
    }

    pub fn gruvbox_dark() -> Self {
        Self {
            name: "Gruvbox Dark".to_string(),
            background: Some(Color::Rgb(40, 40, 40)), // Bg0
            title_try: Color::Rgb(251, 73, 52),           // Red
            title_rs: Color::Rgb(250, 189, 47),           // Yellow
            search_title: Color::Rgb(184, 187, 38),       // Green
            search_border: Color::Rgb(168, 153, 132),     // Grey
            folder_title: Color::Rgb(250, 189, 47),       // Yellow
            folder_border: Color::Rgb(168, 153, 132),     // Grey
            disk_title: Color::Rgb(254, 128, 25),         // Orange
            disk_border: Color::Rgb(168, 153, 132),       // Grey
            preview_title: Color::Rgb(131, 165, 152),     // Aqua
            preview_border: Color::Rgb(168, 153, 132),    // Grey
            legends_title: Color::Rgb(211, 134, 155),     // Purple
            legends_border: Color::Rgb(168, 153, 132),    // Grey
            list_date: Color::Rgb(146, 131, 116),         // Grey
            list_highlight_bg: Color::Rgb(80, 73, 69),    // Bg2
            list_highlight_fg: Color::Rgb(235, 219, 178), // Fg
            helpers_colors: Color::Rgb(168, 153, 132),    // Grey
            status_message: Color::Rgb(215, 153, 33),     // Orange
            popup_bg: Color::Rgb(40, 40, 40),             // Bg0
            popup_text: Color::Rgb(251, 73, 52),          // Red
            // Gruvbox icon colors
            icon_rust: Color::Rgb(254, 128, 25),     // Orange
            icon_maven: Color::Rgb(251, 73, 52),     // Red
            icon_flutter: Color::Rgb(131, 165, 152), // Aqua
            icon_go: Color::Rgb(131, 165, 152),      // Aqua
            icon_python: Color::Rgb(250, 189, 47),   // Yellow
            icon_mise: Color::Rgb(254, 128, 25),     // Orange
            icon_worktree: Color::Rgb(184, 187, 38), // Green
            icon_worktree_lock: Color::Rgb(168, 153, 132), // Grey
            icon_gitmodules: Color::Rgb(211, 134, 155), // Purple
            icon_git: Color::Rgb(251, 73, 52),       // Red
            icon_folder: Color::Rgb(250, 189, 47),   // Yellow
            icon_file: Color::Rgb(146, 131, 116),    // Grey
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".to_string(),
            background: Some(Color::Rgb(46, 52, 64)), // Polar Night 0
            title_try: Color::Rgb(136, 192, 208), // Frost Cyan
            title_rs: Color::Rgb(191, 97, 106),   // Aurora Red
            search_title: Color::Rgb(163, 190, 140), // Aurora Green
            search_border: Color::Rgb(76, 86, 106), // Polar Night 2
            folder_title: Color::Rgb(235, 203, 139), // Aurora Yellow
            folder_border: Color::Rgb(76, 86, 106), // Polar Night 2
            disk_title: Color::Rgb(208, 135, 112), // Aurora Orange
            disk_border: Color::Rgb(76, 86, 106), // Polar Night 2
            preview_title: Color::Rgb(136, 192, 208), // Frost Cyan
            preview_border: Color::Rgb(76, 86, 106), // Polar Night 2
            legends_title: Color::Rgb(180, 142, 173), // Aurora Purple
            legends_border: Color::Rgb(76, 86, 106), // Polar Night 2
            list_date: Color::Rgb(216, 222, 233), // Snow Storm
            list_highlight_bg: Color::Rgb(67, 76, 94), // Polar Night 3
            list_highlight_fg: Color::Rgb(236, 239, 244), // Snow Storm 3
            helpers_colors: Color::Rgb(76, 86, 106), // Polar Night 2
            status_message: Color::Rgb(235, 203, 139), // Aurora Yellow
            popup_bg: Color::Rgb(46, 52, 64),     // Polar Night 0
            popup_text: Color::Rgb(191, 97, 106), // Aurora Red
            // Nord icon colors
            icon_rust: Color::Rgb(208, 135, 112), // Aurora Orange
            icon_maven: Color::Rgb(191, 97, 106), // Aurora Red
            icon_flutter: Color::Rgb(136, 192, 208), // Frost Cyan
            icon_go: Color::Rgb(136, 192, 208),   // Frost Cyan
            icon_python: Color::Rgb(235, 203, 139), // Aurora Yellow
            icon_mise: Color::Rgb(208, 135, 112), // Aurora Orange
            icon_worktree: Color::Rgb(163, 190, 140), // Aurora Green
            icon_worktree_lock: Color::Rgb(216, 222, 233), // Snow Storm
            icon_gitmodules: Color::Rgb(180, 142, 173), // Aurora Purple
            icon_git: Color::Rgb(191, 97, 106),   // Aurora Red
            icon_folder: Color::Rgb(235, 203, 139), // Aurora Yellow
            icon_file: Color::Rgb(216, 222, 233), // Snow Storm
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".to_string(),
            background: Some(Color::Rgb(26, 27, 38)), // Bg
            title_try: Color::Rgb(122, 162, 247),         // Blue
            title_rs: Color::Rgb(247, 118, 142),          // Red
            search_title: Color::Rgb(158, 206, 106),      // Green
            search_border: Color::Rgb(86, 95, 137),       // Comment
            folder_title: Color::Rgb(224, 175, 104),      // Yellow
            folder_border: Color::Rgb(86, 95, 137),       // Comment
            disk_title: Color::Rgb(255, 158, 100),        // Orange
            disk_border: Color::Rgb(86, 95, 137),         // Comment
            preview_title: Color::Rgb(125, 207, 255),     // Cyan
            preview_border: Color::Rgb(86, 95, 137),      // Comment
            legends_title: Color::Rgb(187, 154, 247),     // Purple
            legends_border: Color::Rgb(86, 95, 137),      // Comment
            list_date: Color::Rgb(169, 177, 214),         // Fg
            list_highlight_bg: Color::Rgb(65, 72, 104),   // Terminal Black
            list_highlight_fg: Color::Rgb(192, 202, 245), // Terminal White
            helpers_colors: Color::Rgb(86, 95, 137),      // Comment
            status_message: Color::Rgb(224, 175, 104),    // Yellow
            popup_bg: Color::Rgb(26, 27, 38),             // Bg
            popup_text: Color::Rgb(247, 118, 142),        // Red
            // Tokyo Night icon colors
            icon_rust: Color::Rgb(255, 158, 100),     // Orange
            icon_maven: Color::Rgb(247, 118, 142),    // Red
            icon_flutter: Color::Rgb(125, 207, 255),  // Cyan
            icon_go: Color::Rgb(125, 207, 255),       // Cyan
            icon_python: Color::Rgb(224, 175, 104),   // Yellow
            icon_mise: Color::Rgb(255, 158, 100),     // Orange
            icon_worktree: Color::Rgb(158, 206, 106), // Green
            icon_worktree_lock: Color::Rgb(169, 177, 214), // Fg
            icon_gitmodules: Color::Rgb(187, 154, 247), // Purple
            icon_git: Color::Rgb(247, 118, 142),      // Red
            icon_folder: Color::Rgb(224, 175, 104),   // Yellow
            icon_file: Color::Rgb(169, 177, 214),     // Fg
        }
    }

    pub fn one_dark_pro() -> Self {
        Self {
            name: "One Dark Pro".to_string(),
            background: Some(Color::Rgb(40, 44, 52)), // Bg
            title_try: Color::Rgb(97, 175, 239),          // Blue
            title_rs: Color::Rgb(224, 108, 117),          // Red
            search_title: Color::Rgb(209, 154, 102),      // Orange
            search_border: Color::Rgb(92, 99, 112),       // Comment
            folder_title: Color::Rgb(152, 195, 121),      // Green
            folder_border: Color::Rgb(92, 99, 112),       // Comment
            disk_title: Color::Rgb(229, 192, 123),        // Yellow
            disk_border: Color::Rgb(92, 99, 112),         // Comment
            preview_title: Color::Rgb(86, 182, 194),      // Cyan
            preview_border: Color::Rgb(92, 99, 112),      // Comment
            legends_title: Color::Rgb(198, 120, 221),     // Purple
            legends_border: Color::Rgb(92, 99, 112),      // Comment
            list_date: Color::Rgb(171, 178, 191),         // Fg
            list_highlight_bg: Color::Rgb(62, 68, 81),    // Selection
            list_highlight_fg: Color::Rgb(220, 223, 228), // Bright Fg
            helpers_colors: Color::Rgb(92, 99, 112),      // Comment
            status_message: Color::Rgb(229, 192, 123),    // Yellow
            popup_bg: Color::Rgb(40, 44, 52),             // Bg
            popup_text: Color::Rgb(224, 108, 117),        // Red
            // One Dark Pro icon colors
            icon_rust: Color::Rgb(209, 154, 102),     // Orange
            icon_maven: Color::Rgb(224, 108, 117),    // Red
            icon_flutter: Color::Rgb(86, 182, 194),   // Cyan
            icon_go: Color::Rgb(86, 182, 194),        // Cyan
            icon_python: Color::Rgb(229, 192, 123),   // Yellow
            icon_mise: Color::Rgb(209, 154, 102),     // Orange
            icon_worktree: Color::Rgb(152, 195, 121), // Green
            icon_worktree_lock: Color::Rgb(171, 178, 191), // Fg
            icon_gitmodules: Color::Rgb(198, 120, 221), // Purple
            icon_git: Color::Rgb(224, 108, 117),      // Red
            icon_folder: Color::Rgb(229, 192, 123),   // Yellow
            icon_file: Color::Rgb(171, 178, 191),     // Fg
        }
    }

    pub fn everforest() -> Self {
        Self {
            name: "Everforest".to_string(),
            background: Some(Color::Rgb(45, 51, 48)), // Bg
            title_try: Color::Rgb(127, 187, 179),         // Aqua
            title_rs: Color::Rgb(230, 126, 128),          // Red
            search_title: Color::Rgb(230, 152, 117),      // Orange
            search_border: Color::Rgb(127, 132, 120),     // Grey
            folder_title: Color::Rgb(167, 192, 128),      // Green
            folder_border: Color::Rgb(127, 132, 120),     // Grey
            disk_title: Color::Rgb(219, 188, 127),        // Yellow
            disk_border: Color::Rgb(127, 132, 120),       // Grey
            preview_title: Color::Rgb(127, 187, 179),     // Aqua
            preview_border: Color::Rgb(127, 132, 120),    // Grey
            legends_title: Color::Rgb(214, 153, 182),     // Purple
            legends_border: Color::Rgb(127, 132, 120),    // Grey
            list_date: Color::Rgb(211, 198, 170),         // Fg
            list_highlight_bg: Color::Rgb(80, 88, 77),    // Bg Visual
            list_highlight_fg: Color::Rgb(211, 198, 170), // Fg
            helpers_colors: Color::Rgb(127, 132, 120),    // Grey
            status_message: Color::Rgb(219, 188, 127),    // Yellow
            popup_bg: Color::Rgb(45, 51, 48),             // Bg
            popup_text: Color::Rgb(230, 126, 128),        // Red
            // Everforest icon colors
            icon_rust: Color::Rgb(230, 152, 117),     // Orange
            icon_maven: Color::Rgb(230, 126, 128),    // Red
            icon_flutter: Color::Rgb(127, 187, 179),  // Aqua
            icon_go: Color::Rgb(127, 187, 179),       // Aqua
            icon_python: Color::Rgb(219, 188, 127),   // Yellow
            icon_mise: Color::Rgb(230, 152, 117),     // Orange
            icon_worktree: Color::Rgb(167, 192, 128), // Green
            icon_worktree_lock: Color::Rgb(211, 198, 170), // Fg
            icon_gitmodules: Color::Rgb(214, 153, 182), // Purple
            icon_git: Color::Rgb(230, 126, 128),      // Red
            icon_folder: Color::Rgb(219, 188, 127),   // Yellow
            icon_file: Color::Rgb(211, 198, 170),     // Fg
        }
    }

    pub fn synthwave_84() -> Self {
        Self {
            name: "SynthWave '84".to_string(),
            background: Some(Color::Rgb(38, 29, 53)), // Bg
            title_try: Color::Rgb(54, 244, 244),          // Cyan
            title_rs: Color::Rgb(255, 126, 185),          // Pink
            search_title: Color::Rgb(255, 203, 107),      // Yellow
            search_border: Color::Rgb(129, 91, 164),      // Purple dim
            folder_title: Color::Rgb(114, 241, 177),      // Green
            folder_border: Color::Rgb(129, 91, 164),      // Purple dim
            disk_title: Color::Rgb(255, 203, 107),        // Yellow
            disk_border: Color::Rgb(129, 91, 164),        // Purple dim
            preview_title: Color::Rgb(54, 244, 244),      // Cyan
            preview_border: Color::Rgb(129, 91, 164),     // Purple dim
            legends_title: Color::Rgb(254, 78, 174),      // Hot Pink
            legends_border: Color::Rgb(129, 91, 164),     // Purple dim
            list_date: Color::Rgb(187, 186, 201),         // Fg
            list_highlight_bg: Color::Rgb(57, 43, 75),    // Selection
            list_highlight_fg: Color::Rgb(255, 255, 255), // White
            helpers_colors: Color::Rgb(129, 91, 164),     // Purple dim
            status_message: Color::Rgb(255, 203, 107),    // Yellow
            popup_bg: Color::Rgb(38, 29, 53),             // Bg
            popup_text: Color::Rgb(254, 78, 174),         // Hot Pink
            // SynthWave icon colors
            icon_rust: Color::Rgb(255, 140, 66),      // Orange
            icon_maven: Color::Rgb(255, 126, 185),    // Pink
            icon_flutter: Color::Rgb(54, 244, 244),   // Cyan
            icon_go: Color::Rgb(54, 244, 244),        // Cyan
            icon_python: Color::Rgb(255, 203, 107),   // Yellow
            icon_mise: Color::Rgb(255, 140, 66),      // Orange
            icon_worktree: Color::Rgb(114, 241, 177), // Green
            icon_worktree_lock: Color::Rgb(187, 186, 201), // Fg
            icon_gitmodules: Color::Rgb(254, 78, 174), // Hot Pink
            icon_git: Color::Rgb(255, 126, 185),      // Pink
            icon_folder: Color::Rgb(255, 203, 107),   // Yellow
            icon_file: Color::Rgb(187, 186, 201),     // Fg
        }
    }

    pub fn oled_true_black() -> Self {
        Self {
            name: "OLED True Black".to_string(),
            background: Some(Color::Rgb(0, 0, 0)), // True Black
            title_try: Color::Rgb(0, 200, 255),     // Bright Cyan
            title_rs: Color::Rgb(255, 80, 100),     // Bright Red
            search_title: Color::Rgb(255, 180, 0),  // Orange
            search_border: Color::Rgb(60, 60, 60),  // Dark Grey
            folder_title: Color::Rgb(0, 230, 130),  // Bright Green
            folder_border: Color::Rgb(60, 60, 60),  // Dark Grey
            disk_title: Color::Rgb(255, 220, 0),    // Yellow
            disk_border: Color::Rgb(60, 60, 60),    // Dark Grey
            preview_title: Color::Rgb(0, 200, 255), // Bright Cyan
            preview_border: Color::Rgb(60, 60, 60), // Dark Grey
            legends_title: Color::Rgb(200, 100, 255), // Purple
            legends_border: Color::Rgb(60, 60, 60), // Dark Grey
            list_date: Color::Rgb(180, 180, 180),   // Light Grey
            list_highlight_bg: Color::Rgb(30, 30, 30), // Near Black
            list_highlight_fg: Color::Rgb(255, 255, 255), // White
            helpers_colors: Color::Rgb(100, 100, 100), // Grey
            status_message: Color::Rgb(255, 220, 0), // Yellow
            popup_bg: Color::Rgb(0, 0, 0),          // True Black
            popup_text: Color::Rgb(255, 80, 100),   // Bright Red
            // OLED icon colors - vibrant for contrast
            icon_rust: Color::Rgb(255, 120, 50),  // Bright Orange
            icon_maven: Color::Rgb(255, 80, 100), // Bright Red
            icon_flutter: Color::Rgb(0, 200, 255), // Bright Cyan
            icon_go: Color::Rgb(0, 200, 255),     // Bright Cyan
            icon_python: Color::Rgb(255, 220, 0), // Yellow
            icon_mise: Color::Rgb(255, 180, 0),   // Orange
            icon_worktree: Color::Rgb(0, 230, 130), // Bright Green
            icon_worktree_lock: Color::Rgb(180, 180, 180), // Light Grey
            icon_gitmodules: Color::Rgb(200, 100, 255), // Purple
            icon_git: Color::Rgb(255, 80, 100),   // Bright Red
            icon_folder: Color::Rgb(255, 220, 0), // Yellow
            icon_file: Color::Rgb(180, 180, 180), // Light Grey
        }
    }

    pub fn silver_gray() -> Self {
        Self {
            name: "Silver Gray".to_string(),
            background: Some(Color::Rgb(47, 47, 47)), // Dark Bg
            title_try: Color::Rgb(100, 149, 237), // Cornflower Blue
            title_rs: Color::Rgb(205, 92, 92),    // Indian Red
            search_title: Color::Rgb(218, 165, 32), // Goldenrod
            search_border: Color::Rgb(128, 128, 128), // Gray
            folder_title: Color::Rgb(144, 238, 144), // Light Green
            folder_border: Color::Rgb(128, 128, 128), // Gray
            disk_title: Color::Rgb(240, 230, 140), // Khaki
            disk_border: Color::Rgb(128, 128, 128), // Gray
            preview_title: Color::Rgb(176, 196, 222), // Light Steel Blue
            preview_border: Color::Rgb(128, 128, 128), // Gray
            legends_title: Color::Rgb(186, 85, 211), // Medium Orchid
            legends_border: Color::Rgb(128, 128, 128), // Gray
            list_date: Color::Rgb(192, 192, 192), // Silver
            list_highlight_bg: Color::Rgb(70, 70, 70), // Dark Gray
            list_highlight_fg: Color::Rgb(245, 245, 245), // White Smoke
            helpers_colors: Color::Rgb(128, 128, 128), // Gray
            status_message: Color::Rgb(240, 230, 140), // Khaki
            popup_bg: Color::Rgb(47, 47, 47),     // Dark Bg
            popup_text: Color::Rgb(205, 92, 92),  // Indian Red
            // Silver Gray icon colors
            icon_rust: Color::Rgb(210, 105, 30),      // Chocolate
            icon_maven: Color::Rgb(205, 92, 92),      // Indian Red
            icon_flutter: Color::Rgb(100, 149, 237),  // Cornflower Blue
            icon_go: Color::Rgb(176, 196, 222),       // Light Steel Blue
            icon_python: Color::Rgb(240, 230, 140),   // Khaki
            icon_mise: Color::Rgb(218, 165, 32),      // Goldenrod
            icon_worktree: Color::Rgb(144, 238, 144), // Light Green
            icon_worktree_lock: Color::Rgb(192, 192, 192), // Silver
            icon_gitmodules: Color::Rgb(186, 85, 211), // Medium Orchid
            icon_git: Color::Rgb(205, 92, 92),        // Indian Red
            icon_folder: Color::Rgb(240, 230, 140),   // Khaki
            icon_file: Color::Rgb(192, 192, 192),     // Silver
        }
    }

    pub fn black_and_white() -> Self {
        Self {
            name: "Black & White".to_string(),
            background: Some(Color::Black),
            title_try: Color::White,
            title_rs: Color::White,
            search_title: Color::White,
            search_border: Color::Gray,
            folder_title: Color::White,
            folder_border: Color::Gray,
            disk_title: Color::White,
            disk_border: Color::Gray,
            preview_title: Color::White,
            preview_border: Color::Gray,
            legends_title: Color::White,
            legends_border: Color::Gray,
            list_date: Color::Gray,
            list_highlight_bg: Color::White,
            list_highlight_fg: Color::Black,
            helpers_colors: Color::Gray,
            status_message: Color::White,
            popup_bg: Color::Black,
            popup_text: Color::White,
            // Black & White icon colors
            icon_rust: Color::White,
            icon_maven: Color::White,
            icon_flutter: Color::White,
            icon_go: Color::White,
            icon_python: Color::White,
            icon_mise: Color::Gray,
            icon_worktree: Color::White,
            icon_worktree_lock: Color::Gray,
            icon_gitmodules: Color::Gray,
            icon_git: Color::White,
            icon_folder: Color::White,
            icon_file: Color::Gray,
        }
    }

    pub fn matrix() -> Self {
        Self {
            name: "Matrix".to_string(),
            background: Some(Color::Rgb(0, 10, 0)), // Almost black with green tint
            title_try: Color::Rgb(0, 255, 65),            // Matrix green
            title_rs: Color::Rgb(0, 200, 50),             // Darker green
            search_title: Color::Rgb(0, 255, 65),         // Matrix green
            search_border: Color::Rgb(0, 100, 30),        // Dark green
            folder_title: Color::Rgb(0, 255, 65),         // Matrix green
            folder_border: Color::Rgb(0, 100, 30),        // Dark green
            disk_title: Color::Rgb(0, 255, 65),           // Matrix green
            disk_border: Color::Rgb(0, 100, 30),          // Dark green
            preview_title: Color::Rgb(0, 255, 65),        // Matrix green
            preview_border: Color::Rgb(0, 100, 30),       // Dark green
            legends_title: Color::Rgb(0, 200, 50),        // Darker green
            legends_border: Color::Rgb(0, 100, 30),       // Dark green
            list_date: Color::Rgb(0, 150, 40),            // Muted green
            list_highlight_bg: Color::Rgb(0, 80, 25),     // Selection green
            list_highlight_fg: Color::Rgb(0, 255, 65),    // Bright green
            helpers_colors: Color::Rgb(0, 150, 40),       // Muted green
            status_message: Color::Rgb(0, 255, 65),       // Matrix green
            popup_bg: Color::Rgb(0, 10, 0),               // Almost black with green tint
            popup_text: Color::Rgb(0, 255, 65),           // Matrix green
            // Matrix icon colors (all shades of green)
            icon_rust: Color::Rgb(0, 255, 65),
            icon_maven: Color::Rgb(0, 220, 55),
            icon_flutter: Color::Rgb(0, 200, 50),
            icon_go: Color::Rgb(0, 180, 45),
            icon_python: Color::Rgb(0, 255, 65),
            icon_mise: Color::Rgb(0, 150, 40),
            icon_worktree: Color::Rgb(0, 200, 50),
            icon_worktree_lock: Color::Rgb(0, 120, 35),
            icon_gitmodules: Color::Rgb(0, 180, 45),
            icon_git: Color::Rgb(0, 255, 65),
            icon_folder: Color::Rgb(0, 220, 55),
            icon_file: Color::Rgb(0, 150, 40),
        }
    }

    pub fn tron() -> Self {
        Self {
            name: "Tron".to_string(),
            background: Some(Color::Rgb(0, 10, 15)), // Almost black with blue tint
            title_try: Color::Rgb(0, 255, 255),           // Cyan neon
            title_rs: Color::Rgb(255, 150, 0),            // Orange accent
            search_title: Color::Rgb(0, 255, 255),        // Cyan neon
            search_border: Color::Rgb(0, 150, 180),       // Darker cyan
            folder_title: Color::Rgb(0, 255, 255),        // Cyan neon
            folder_border: Color::Rgb(0, 150, 180),       // Darker cyan
            disk_title: Color::Rgb(255, 150, 0),          // Orange accent
            disk_border: Color::Rgb(0, 150, 180),         // Darker cyan
            preview_title: Color::Rgb(0, 255, 255),       // Cyan neon
            preview_border: Color::Rgb(0, 150, 180),      // Darker cyan
            legends_title: Color::Rgb(0, 200, 220),       // Light cyan
            legends_border: Color::Rgb(0, 150, 180),      // Darker cyan
            list_date: Color::Rgb(0, 180, 200),           // Muted cyan
            list_highlight_bg: Color::Rgb(0, 80, 100),    // Selection
            list_highlight_fg: Color::Rgb(0, 255, 255),   // Cyan neon
            helpers_colors: Color::Rgb(0, 150, 180),      // Darker cyan
            status_message: Color::Rgb(255, 150, 0),      // Orange accent
            popup_bg: Color::Rgb(0, 10, 15),              // Almost black with blue tint
            popup_text: Color::Rgb(0, 255, 255),          // Cyan neon
            // Tron icon colors
            icon_rust: Color::Rgb(255, 150, 0),           // Orange
            icon_maven: Color::Rgb(255, 100, 0),          // Dark orange
            icon_flutter: Color::Rgb(0, 255, 255),        // Cyan
            icon_go: Color::Rgb(0, 220, 230),             // Light cyan
            icon_python: Color::Rgb(255, 200, 0),         // Yellow-orange
            icon_mise: Color::Rgb(255, 150, 0),           // Orange
            icon_worktree: Color::Rgb(0, 255, 255),       // Cyan
            icon_worktree_lock: Color::Rgb(0, 150, 180),  // Darker cyan
            icon_gitmodules: Color::Rgb(0, 200, 220),     // Light cyan
            icon_git: Color::Rgb(255, 150, 0),            // Orange
            icon_folder: Color::Rgb(0, 255, 255),         // Cyan
            icon_file: Color::Rgb(0, 180, 200),           // Muted cyan
        }
    }

    pub fn catppuccin_macchiato() -> Self {
        Self {
            name: "Catppuccin Macchiato".to_string(),
            background: Some(Color::Rgb(36, 39, 58)), // Base
            title_try: Color::Rgb(138, 173, 244),         // Blue
            title_rs: Color::Rgb(238, 153, 160),          // Maroon
            search_title: Color::Rgb(245, 169, 127),      // Peach
            search_border: Color::Rgb(147, 154, 183),     // Overlay1
            folder_title: Color::Rgb(166, 218, 149),      // Green
            folder_border: Color::Rgb(147, 154, 183),     // Overlay1
            disk_title: Color::Rgb(238, 212, 159),        // Yellow
            disk_border: Color::Rgb(147, 154, 183),       // Overlay1
            preview_title: Color::Rgb(138, 173, 244),     // Blue
            preview_border: Color::Rgb(147, 154, 183),    // Overlay1
            legends_title: Color::Rgb(198, 160, 246),     // Mauve
            legends_border: Color::Rgb(147, 154, 183),    // Overlay1
            list_date: Color::Rgb(165, 173, 203),         // Subtext0
            list_highlight_bg: Color::Rgb(91, 96, 120),   // Surface2
            list_highlight_fg: Color::Rgb(202, 211, 245), // Text
            helpers_colors: Color::Rgb(147, 154, 183),    // Overlay1
            status_message: Color::Rgb(238, 212, 159),    // Yellow
            popup_bg: Color::Rgb(36, 39, 58),             // Base
            popup_text: Color::Rgb(237, 135, 150),        // Red
            // Catppuccin Macchiato icon colors
            icon_rust: Color::Rgb(245, 169, 127),     // Peach
            icon_maven: Color::Rgb(237, 135, 150),    // Red
            icon_flutter: Color::Rgb(138, 173, 244),  // Blue
            icon_go: Color::Rgb(139, 213, 202),       // Teal
            icon_python: Color::Rgb(238, 212, 159),   // Yellow
            icon_mise: Color::Rgb(245, 169, 127),     // Peach
            icon_worktree: Color::Rgb(166, 218, 149), // Green
            icon_worktree_lock: Color::Rgb(165, 173, 203), // Subtext0
            icon_gitmodules: Color::Rgb(198, 160, 246), // Mauve
            icon_git: Color::Rgb(237, 135, 150),      // Red
            icon_folder: Color::Rgb(238, 212, 159),   // Yellow
            icon_file: Color::Rgb(165, 173, 203),     // Subtext0
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
