pub mod svg;

pub struct Chart {
    stats: Vec<(String, i32)>,
    colors: Vec<&'static str>,
}

impl Chart {
    pub fn new(stats: Vec<(String, i32)>, colors: Option<Vec<&'static str>>) -> Self {
        let default_colors = vec!["#eeeeee", "#c6e48b", "#7bc96f", "#239a3b", "#196127"];
        Chart {
            stats,
            colors: colors.unwrap_or(default_colors),
        }
    }

    pub fn render(&self, format: &str) -> Result<String, String> {
        match format {
            "svg" => Ok(svg::render_svg(&self)),
            "svg_square" => Ok(svg::render_svg_square(&self)),
            _ => Err(format!("Format {} is unsupported.", format)),
        }
    }
}

pub const COLOR_SCHEMES: &[(&str, &[&str])] = &[
    ("default", &["#eeeeee", "#c6e48b", "#7bc96f", "#239a3b", "#196127"]),
    ("old", &["#eeeeee", "#d6e685", "#8cc665", "#44a340", "#1e6823"]),
    ("halloween", &["#EEEEEE", "#FFEE4A", "#FFC501", "#FE9600", "#03001C"]),
];
