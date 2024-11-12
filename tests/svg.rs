use super::*;

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
fn test_svg_add_points() {
    let stats = vec![("2023-07-18".to_string(), 5)];
    let chart = Chart::new(stats, None);
    let svg = chart.render("svg").unwrap();
    assert!(svg.contains("data-score"));
    assert!(svg.contains("data-date"));
}

#[test]
fn test_svg_add_weekdays() {
    let stats = vec![("2023-07-18".to_string(), 5)];
    let chart = Chart::new(stats, None);
    let svg = chart.render("svg").unwrap();
    assert!(svg.contains("Mon"));
    assert!(svg.contains("Wed"));
    assert!(svg.contains("Fri"));
}

#[test]
fn test_svg_add_months() {
    let stats = vec![("2023-07-18".to_string(), 5)];
    let chart = Chart::new(stats, None);
    let svg = chart.render("svg").unwrap();
    assert!(svg.contains("Jan"));
    assert!(svg.contains("Feb"));
    assert!(svg.contains("Mar"));
}
