use reqwest;
use scraper::{Html, Selector};

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

    pub fn render(&self) -> Result<String, String> {
        Ok(svg::render_svg(&self))
    }
}

pub const COLOR_SCHEMES: &[(&str, &[&str])] = &[
    (
        "default",
        &["#eeeeee", "#c6e48b", "#7bc96f", "#239a3b", "#196127"],
    ),
    (
        "old",
        &["#eeeeee", "#d6e685", "#8cc665", "#44a340", "#1e6823"],
    ),
    (
        "halloween",
        &["#EEEEEE", "#FFEE4A", "#FFC501", "#FE9600", "#03001C"],
    ),
];

pub fn fetch_github_stats(
    username: &str,
) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
    // GitHub's contribution graph data endpoint
    let url = format!("https://github.com/users/{}/contributions", username);

    // Fetch the HTML content
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "githubchart-rust")
        .send()?;

    // Check status code
    if !response.status().is_success() {
        return Err(format!(
            "Failed loading data from GitHub: {} {}",
            url,
            response.status()
        )
        .into());
    }

    let html_content = response.text()?;
    let document = Html::parse_document(&html_content);

    // Select contribution calendar cells
    let cell_selector = Selector::parse("td.ContributionCalendar-day").unwrap();

    let mut stats = Vec::new();

    for element in document.select(&cell_selector) {
        if let (Some(date), Some(count_str)) = (
            element.value().attr("data-date"),
            element.value().attr("data-level"),
        ) {
            if let Ok(count) = count_str.parse::<i32>() {
                // println!("fetch_github_stats > date: {}, count: {}", date, count);
                stats.push((date.to_string(), count));
            }
        }
    }

    // Sort by date
    stats.sort_by(|a, b| a.0.cmp(&b.0));

    if stats.is_empty() {
        println!("Warning: No contribution data found for user {}", username);
    }

    Ok(stats)
}

#[cfg(test)]
mod tests;
