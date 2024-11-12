pub mod svg {
    use crate::Chart;

    pub fn render_svg(chart: &Chart) -> String {
        let grid = matrix(&chart.stats);
        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" version="1.1" xmlns="http://www.w3.org/2000/svg">"#,
            (CUBE_SIZE * grid[0].len()) + X_PAD,
            (CUBE_SIZE * grid.len()) + Y_PAD
        ));
        svg_add_points(&grid, &mut svg);
        svg_add_weekdays(&mut svg);
        svg_add_months(&mut svg);
        svg.push_str("</svg>");
        svg
    }

    pub fn render_svg_square(chart: &Chart) -> String {
        let grid = matrix(&chart.stats);
        let grid = grid.iter().map(|row| &row[0..7]).collect::<Vec<_>>();
        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" version="1.1" xmlns="http://www.w3.org/2000/svg">"#,
            (CUBE_SIZE * grid[0].len()) - 2,
            (CUBE_SIZE * grid.len()) - 2
        ));
        svg_add_points(&grid, &mut svg);
        svg.push_str("</svg>");
        svg
    }

    const CUBE_SIZE: usize = 12;
    const X_PAD: usize = 27;
    const Y_PAD: usize = 20;

    fn matrix(stats: &[(String, i32)]) -> Vec<Vec<Point>> {
        let mut grid = vec![vec![Point::default(); 7]; (stats.len() + 6) / 7];
        for (i, (date, score)) in stats.iter().enumerate() {
            let x = i / 7;
            let y = i % 7;
            grid[x][y] = Point {
                date: date.clone(),
                score: *score,
            };
        }
        grid
    }

    #[derive(Default, Clone)]
    struct Point {
        date: String,
        score: i32,
    }

    fn svg_add_points(grid: &[Vec<Point>], svg: &mut String) {
        for (x, row) in grid.iter().enumerate() {
            for (y, point) in row.iter().enumerate() {
                if point.score == -1 {
                    continue;
                }
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="10" height="10" style="fill:{};" data-score="{}" data-date="{}"/>"#,
                    (x * CUBE_SIZE) + X_PAD,
                    (y * CUBE_SIZE) + Y_PAD,
                    point_color(point.score),
                    point.score,
                    point.date
                ));
            }
        }
    }

    fn svg_add_weekdays(svg: &mut String) {
        let weekdays = ["Mon", "Wed", "Fri"];
        for (i, &day) in weekdays.iter().enumerate() {
            svg.push_str(&format!(
                r#"<text x="0" y="{}" style="fill:#767676;text-anchor:start;text-align:center;font-family:-apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol';white-space:nowrap;font-size:9px;display:{};">{}</text>"#,
                (CUBE_SIZE * (i * 2 + 1)) + 28,
                if i == 1 { "block" } else { "none" },
                day
            ));
        }
    }

    fn svg_add_months(svg: &mut String) {
        let months = [
            ("Jan", 0),
            ("Feb", 31),
            ("Mar", 59),
            ("Apr", 90),
            ("May", 120),
            ("Jun", 151),
            ("Jul", 181),
            ("Aug", 212),
            ("Sep", 243),
            ("Oct", 273),
            ("Nov", 304),
            ("Dec", 334),
        ];
        for &(month, offset) in &months {
            if offset > 50 {
                continue;
            }
            let x = (CUBE_SIZE * offset / 7) + X_PAD;
            svg.push_str(&format!(
                r#"<text x="{}" y="10" style="fill:#767676;text-anchor:start;text-align:center;font-family:-apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol';white-space:nowrap;font-size:10px;">{}</text>"#,
                x,
                month
            ));
        }
    }

    fn point_color(score: i32) -> &'static str {
        match score {
            0 => "#eeeeee",
            1..=3 => "#c6e48b",
            4..=6 => "#7bc96f",
            7..=9 => "#239a3b",
            _ => "#196127",
        }
    }
}
