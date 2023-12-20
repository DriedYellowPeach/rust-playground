use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            r#"
            Press `Esc`, `Ctrl-C` or `q` to stop running.
            Press `j` and `k` to increment and decrement the counter respectively.
            Counter: {}
            "#,
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ),
        f.size(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_raw_string_output() {
        let s = format!(
            "
            hello
            world
            {}
            ",
            "kevin"
        );

        println!("{s}");
    }
}
