use super::*;

#[test]
fn test_new_chart() {
    let stats = vec![("2023-07-18".to_string(), 5)];
    let chart = Chart::new(stats.clone(), None);
    assert_eq!(chart.stats, stats);
    assert_eq!(
        chart.colors,
        vec!["#eeeeee", "#c6e48b", "#7bc96f", "#239a3b", "#196127"]
    );
}

#[test]
fn test_render_svg() {
    let stats = vec![("2023-07-18".to_string(), 5)];
    let chart = Chart::new(stats, None);
    let svg = chart.render("svg").unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
}

#[test]
fn test_render_svg_square() {
    let stats = vec![("2023-07-18".to_string(), 5)];
    let chart = Chart::new(stats, None);
    let svg_square = chart.render("svg_square").unwrap();
    assert!(svg_square.contains("<svg"));
    assert!(svg_square.contains("</svg>"));
}

#[test]
fn test_color_schemes() {
    let stats = vec![("2023-07-18".to_string(), 5)];
    let chart = Chart::new(stats.clone(), Some(vec!["#FF0000", "#00FF00", "#0000FF"]));
    assert_eq!(chart.colors, vec!["#FF0000", "#00FF00", "#0000FF"]);

    let chart_default = Chart::new(stats, None);
    assert_eq!(
        chart_default.colors,
        vec!["#eeeeee", "#c6e48b", "#7bc96f", "#239a3b", "#196127"]
    );
}
