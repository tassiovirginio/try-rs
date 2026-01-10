use ratatui::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub title_try: Color,
    pub title_rs: Color,
    pub search_box: Color,
    pub list_date: Color,
    pub list_highlight_bg: Color,
    pub list_highlight_fg: Color,
    pub help_text: Color,
    pub status_message: Color,
    pub popup_bg: Color,
    pub popup_text: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "Catppuccin Mocha".to_string(),
            title_try: Color::Rgb(137, 180, 250),         // Blue
            title_rs: Color::Rgb(243, 139, 168),          // Red
            search_box: Color::Rgb(250, 179, 135),        // Peach
            list_date: Color::Rgb(166, 173, 200),         // Subtext0
            list_highlight_bg: Color::Rgb(88, 91, 112),   // Surface2
            list_highlight_fg: Color::Rgb(205, 214, 244), // Text
            help_text: Color::Rgb(147, 153, 178),         // Overlay2
            status_message: Color::Rgb(249, 226, 175),    // Yellow
            popup_bg: Color::Rgb(30, 30, 46),             // Base
            popup_text: Color::Rgb(243, 139, 168),        // Red
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "Dracula".to_string(),
            title_try: Color::Rgb(189, 147, 249),      // Purple
            title_rs: Color::Rgb(255, 121, 198),       // Pink
            search_box: Color::Rgb(255, 184, 108),     // Orange
            list_date: Color::Rgb(139, 233, 253),      // Cyan
            list_highlight_bg: Color::Rgb(68, 71, 90), // Selection
            list_highlight_fg: Color::Rgb(248, 248, 242), // Foreground
            help_text: Color::Rgb(98, 114, 164),       // Comment
            status_message: Color::Rgb(241, 250, 140), // Yellow
            popup_bg: Color::Rgb(40, 42, 54),          // Background
            popup_text: Color::Rgb(255, 85, 85),       // Red
        }
    }

    pub fn jetbrains_darcula() -> Self {
        Self {
            name: "JetBrains Darcula".to_string(),
            title_try: Color::Rgb(78, 124, 238),        // Blueish
            title_rs: Color::Rgb(204, 120, 50),         // Orange
            search_box: Color::Rgb(106, 135, 89),       // Green
            list_date: Color::Rgb(128, 128, 128),       // Grey
            list_highlight_bg: Color::Rgb(33, 66, 131), // Selection
            list_highlight_fg: Color::Rgb(187, 187, 187), // Light Grey
            help_text: Color::Rgb(128, 128, 128),
            status_message: Color::Rgb(255, 198, 109), // Gold
            popup_bg: Color::Rgb(60, 63, 65),          // Bg
            popup_text: Color::Rgb(204, 120, 50),
        }
    }

    pub fn gruvbox_dark() -> Self {
        Self {
            name: "Gruvbox Dark".to_string(),
            title_try: Color::Rgb(251, 73, 52),           // Red
            title_rs: Color::Rgb(250, 189, 47),           // Yellow
            search_box: Color::Rgb(184, 187, 38),         // Green
            list_date: Color::Rgb(146, 131, 116),         // Grey
            list_highlight_bg: Color::Rgb(80, 73, 69),    // Bg2
            list_highlight_fg: Color::Rgb(235, 219, 178), // Fg
            help_text: Color::Rgb(168, 153, 132),
            status_message: Color::Rgb(215, 153, 33), // Orange
            popup_bg: Color::Rgb(40, 40, 40),         // Bg0
            popup_text: Color::Rgb(251, 73, 52),
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".to_string(),
            title_try: Color::Rgb(136, 192, 208),  // Frost Cyan
            title_rs: Color::Rgb(191, 97, 106),    // Aurora Red
            search_box: Color::Rgb(163, 190, 140), // Aurora Green
            list_date: Color::Rgb(216, 222, 233),  // Snow Storm
            list_highlight_bg: Color::Rgb(67, 76, 94), // Polar Night 3
            list_highlight_fg: Color::Rgb(236, 239, 244), // Snow Storm 3
            help_text: Color::Rgb(76, 86, 106),    // Polar Night 2
            status_message: Color::Rgb(235, 203, 139), // Aurora Yellow
            popup_bg: Color::Rgb(46, 52, 64),      // Polar Night 0
            popup_text: Color::Rgb(191, 97, 106),
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".to_string(),
            title_try: Color::Rgb(122, 162, 247),         // Blue
            title_rs: Color::Rgb(247, 118, 142),          // Red
            search_box: Color::Rgb(158, 206, 106),        // Green
            list_date: Color::Rgb(169, 177, 214),         // Fg
            list_highlight_bg: Color::Rgb(65, 72, 104),   // Terminal Black
            list_highlight_fg: Color::Rgb(192, 202, 245), // Terminal White
            help_text: Color::Rgb(86, 95, 137),           // Comment
            status_message: Color::Rgb(224, 175, 104),    // Yellow
            popup_bg: Color::Rgb(26, 27, 38),             // Bg
            popup_text: Color::Rgb(247, 118, 142),
        }
    }

    pub fn all() -> Vec<Theme> {
        vec![
            Theme::catppuccin_mocha(),
            Theme::dracula(),
            Theme::jetbrains_darcula(),
            Theme::gruvbox_dark(),
            Theme::nord(),
            Theme::tokyo_night(),
        ]
    }
}
