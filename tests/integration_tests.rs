use regex::Regex;

use githubchart::{fetch_github_stats, Chart, COLOR_SCHEMES};

#[test]
fn test_github_stats_fetching() {
    // Note: This test requires internet connection
    match fetch_github_stats("frytg") {
        Ok(stats) => {
            assert!(!stats.is_empty());
            // Check date format
            assert!(Regex::new(r"^\d{4}-\d{2}-\d{2}$")
                .unwrap()
                .is_match(&stats[0].0));
            // Check contribution count is non-negative
            assert!(stats[0].1 >= 0);
        }
        Err(e) => panic!("Failed to fetch stats: {}", e),
    }
}

#[test]
fn test_full_chart_generation() {
    let stats = vec![("2024-01-01".to_string(), 0), ("2024-01-02".to_string(), 5)];
    let chart = Chart::new(stats, Some(COLOR_SCHEMES[0].1.to_vec()));
    let svg = chart.render().expect("Failed to render chart");

    assert!(svg.contains("svg"));
    assert!(svg.contains("rect"));
    assert!(svg.contains("Jan")); // Month label
}
