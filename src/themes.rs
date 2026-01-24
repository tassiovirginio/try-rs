use ratatui::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
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

    pub fn all() -> Vec<Theme> {
        vec![
            Theme::default_theme(),
            Theme::catppuccin_mocha(),
            Theme::dracula(),
            Theme::jetbrains_darcula(),
            Theme::gruvbox_dark(),
            Theme::nord(),
            Theme::tokyo_night(),
        ]
    }
}
