use crate::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

/// Shape to draw a line from (x1, y1) to (x2, y2) with the given color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub color: Color,
}

impl Line {
    /// Create a new line from (x1, y1) to (x2, y2) with the given color
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, color: Color) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            color,
        }
    }
}

impl Shape for Line {
    fn draw(&self, painter: &mut Painter) {
        let Some((x1, y1)) = painter.get_point(self.x1, self.y1) else {
            return;
        };
        let Some((x2, y2)) = painter.get_point(self.x2, self.y2) else {
            return;
        };
        let (dx, x_range) = if x2 >= x1 {
            (x2 - x1, x1..=x2)
        } else {
            (x1 - x2, x2..=x1)
        };
        let (dy, y_range) = if y2 >= y1 {
            (y2 - y1, y1..=y2)
        } else {
            (y1 - y2, y2..=y1)
        };

        if dx == 0 {
            for y in y_range {
                painter.paint(x1, y, self.color);
            }
        } else if dy == 0 {
            for x in x_range {
                painter.paint(x, y1, self.color);
            }
        } else if dy < dx {
            if x1 > x2 {
                draw_line_low(painter, x2, y2, x1, y1, self.color);
            } else {
                draw_line_low(painter, x1, y1, x2, y2, self.color);
            }
        } else if y1 > y2 {
            draw_line_high(painter, x2, y2, x1, y1, self.color);
        } else {
            draw_line_high(painter, x1, y1, x2, y2, self.color);
        }
    }
}

fn draw_line_low(painter: &mut Painter, x1: usize, y1: usize, x2: usize, y2: usize, color: Color) {
    let dx = (x2 - x1) as isize;
    let dy = (y2 as isize - y1 as isize).abs();
    let mut d = 2 * dy - dx;
    let mut y = y1;
    for x in x1..=x2 {
        painter.paint(x, y, color);
        if d > 0 {
            y = if y1 > y2 {
                y.saturating_sub(1)
            } else {
                y.saturating_add(1)
            };
            d -= 2 * dx;
        }
        d += 2 * dy;
    }
}

fn draw_line_high(painter: &mut Painter, x1: usize, y1: usize, x2: usize, y2: usize, color: Color) {
    let dx = (x2 as isize - x1 as isize).abs();
    let dy = (y2 - y1) as isize;
    let mut d = 2 * dx - dy;
    let mut x = x1;
    for y in y1..=y2 {
        painter.paint(x, y, color);
        if d > 0 {
            x = if x1 > x2 {
                x.saturating_sub(1)
            } else {
                x.saturating_add(1)
            };
            d -= 2 * dy;
        }
        d += 2 * dx;
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::{
        assert_buffer_eq,
        prelude::*,
        widgets::{canvas::Canvas, Widget},
    };

    #[track_caller]
    fn test(line: Line, expected_lines: Vec<&str>) {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|context| {
                context.draw(&line);
            });
        canvas.render(buffer.area, &mut buffer);

        let mut expected = Buffer::with_lines(expected_lines);
        for cell in expected.content.iter_mut() {
            if cell.symbol == "•" {
                cell.set_style(Style::new().red());
            }
        }
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn off_grid() {
        test(
            Line::new(-1.0, -1.0, 10.0, 10.0, Color::Red),
            vec!["          "; 10],
        );
        test(
            Line::new(0.0, 0.0, 11.0, 11.0, Color::Red),
            vec!["          "; 10],
        );
    }

    #[test]
    fn horizontal() {
        test(
            Line::new(0.0, 0.0, 10.0, 0.0, Color::Red),
            vec![
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "••••••••••",
            ],
        );
        test(
            Line::new(10.0, 10.0, 0.0, 10.0, Color::Red),
            vec![
                "••••••••••",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
                "          ",
            ],
        );
    }

    #[test]
    fn vertical() {
        test(
            Line::new(0.0, 0.0, 0.0, 10.0, Color::Red),
            vec!["•         "; 10],
        );
        test(
            Line::new(10.0, 10.0, 10.0, 0.0, Color::Red),
            vec!["         •"; 10],
        );
    }

    #[test]
    fn diagonal() {
        // dy < dx, x1 < x2
        test(
            Line::new(0.0, 0.0, 10.0, 5.0, Color::Red),
            vec![
                "          ",
                "          ",
                "          ",
                "          ",
                "         •",
                "       •• ",
                "     ••   ",
                "   ••     ",
                " ••       ",
                "•         ",
            ],
        );
        // dy < dx, x1 > x2
        test(
            Line::new(10.0, 0.0, 0.0, 5.0, Color::Red),
            vec![
                "          ",
                "          ",
                "          ",
                "          ",
                "•         ",
                " ••       ",
                "   ••     ",
                "     ••   ",
                "       •• ",
                "         •",
            ],
        );
        // dy > dx, y1 < y2
        test(
            Line::new(0.0, 0.0, 5.0, 10.0, Color::Red),
            vec![
                "    •     ",
                "    •     ",
                "   •      ",
                "   •      ",
                "  •       ",
                "  •       ",
                " •        ",
                " •        ",
                "•         ",
                "•         ",
            ],
        );
        // dy > dx, y1 > y2
        test(
            Line::new(0.0, 10.0, 5.0, 0.0, Color::Red),
            vec![
                "•         ",
                "•         ",
                " •        ",
                " •        ",
                "  •       ",
                "  •       ",
                "   •      ",
                "   •      ",
                "    •     ",
                "    •     ",
            ],
        );
    }
}
