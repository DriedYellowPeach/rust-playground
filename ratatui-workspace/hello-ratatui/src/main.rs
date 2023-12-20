use crossterm::{
    event::{self, KeyCode, KeyEventKind, EnableMouseCapture, DisableMouseCapture},
    style,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{CrosstermBackend, Stylize, Terminal},
    text::{Line, Text},
    widgets::{Block, Paragraph},
    Frame,
};
use std::io::stdout;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn ui1(frame: &mut Frame<'_>) {
    let mut step_right = true;
    let rightmost = 10u16;
    let height = frame.size().height;
    let mut offset = 0u16;

    for i in 0..height {
        let area = Rect::new(offset, i, frame.size().width - offset, 1);
        frame.render_widget(Paragraph::new("Hello world"), area);

        match (step_right, offset) {
            (true, _) if offset == rightmost => {
                offset -= 1;
                step_right = false;
            }
            (true, _) => {
                offset += 1;
            }
            (false, _) if offset == 0 => {
                offset += 1;
                step_right = true;
            }
            (false, _) => {
                offset -= 1;
            }
        };
    }
}

#[test]
fn test_ui2() {
    // let mit = (0..10).chain((0..10).rev());
    let mit = (0..10).chain((1..10).rev()).cycle().take(30);
    for i in mit {
        println!("{i}");
    }
}

fn wave_ui(frame: &mut Frame<'_>) {
    use ratatui::style::Color;
    let mit = (0..10)
        .chain((0..10).rev())
        .cycle()
        .take(frame.size().height as usize);

    for (row, col) in mit.enumerate() {
        let area = Rect::new(col, row as u16, "Hello world".len() as u16, 1);
        frame.render_widget(
            Paragraph::new("Hello world")
                .bg(Color::DarkGray)
                .fg(Color::Yellow),
            area,
        );
    }
}

fn ui3(frame: &mut Frame<'_>) {
    use ratatui::widgets::Block;
    use ratatui::widgets::Borders;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new("Bottom").block(Block::new().borders(Borders::ALL)),
        layout[1],
    );
}

fn ui4(frame: &mut Frame<'_>) {
    use ratatui::widgets::{Block, Borders};
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(outer_layout[1]);

    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        outer_layout[0],
    );

    frame.render_widget(
        Paragraph::new("Bottom Left").block(Block::new().borders(Borders::ALL)),
        inner_layout[0],
    );

    frame.render_widget(
        Paragraph::new("Bottom Right").block(Block::new().borders(Borders::ALL)),
        inner_layout[1],
    );
}

fn collapse_border_ui(frame: &mut Frame<'_>) {
    use ratatui::{prelude::*, widgets::*};

    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[1]);

    frame.render_widget(
        Block::new()
            .border_set(symbols::border::PLAIN)
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .title("Left Block"),
        outer_layout[0],
    );

    let top_right_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.horizontal_down,
        ..symbols::border::PLAIN
    };

    frame.render_widget(
        Block::new()
            .border_set(top_right_border_set)
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .title("Top Right Block"),
        right_layout[0],
    );

    let bottom_right_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        bottom_left: symbols::line::NORMAL.horizontal_up,
        ..symbols::border::PLAIN
    };

    frame.render_widget(
        Block::new()
            .border_set(bottom_right_border_set)
            .borders(Borders::ALL)
            .title("Bottom Right Block"),
        right_layout[1],
    );
}

fn ui_style_text(frame: &mut Frame<'_>) {
    use ratatui::{prelude::*, widgets::Borders};
    use std::str::FromStr;
    let mixed_line = Line::from(vec![
        Span::styled("This is mixed", Style::default().fg(Color::Green)),
        Span::styled(
            "styling",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::from("!"),
    ]);
    let text = vec![
        Line::styled(
            "Hello World",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Line::styled(
            "Hello World",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        ),
        Line::styled(
            "Hello World",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::UNDERLINED),
        ),
        Line::styled(
            "Hello World",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::RAPID_BLINK),
        ),
        Line::styled(
            "Hello World",
            Style::default()
                .fg(Color::Rgb(0x48, 0x00, 0x82))
                .add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::REVERSED),
        ),
        Line::styled(
            "Hello World",
            Style::default()
                .fg(Color::Rgb(0x23, 0x44, 0x32))
                .add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::CROSSED_OUT),
        ),
        mixed_line,
        Line::styled("Hello World", Style::default().fg(Color::from_str("#FF0000").unwrap()).bg(Color::from_str("#00FF00").unwrap())),
    ];

    frame.render_widget(
        Paragraph::new(text).block(
            Block::default()
                .title("Some Styled Text")
                .borders(Borders::ALL),
        ),
        frame.size(),
    );
}

fn ui_scrollable_paragraph(frame: &mut Frame<'_>) {
    use ratatui::widgets::Borders;
    let p = Paragraph::new((1..100).map(|idx| format!("{idx}: hello\n")).collect::<String>())
                .scroll((1,0));

    frame.render_widget(p.block(Block::default().title("Scrollable Paragraph").borders(Borders::ALL)), frame.size())

}

fn hello_app() -> Result<()> {
    // stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    crossterm::execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            // let area = frame.size();
            // frame.render_widget(
            //     Paragraph::new("Hello Ratatui!(press 'q' to quit)")
            //         .white()
            //         .on_red(),
            //     area,
            // )

            // wave_ui(frame);
            // ui3(frame);
            // ui4(frame);
            // collapse_border_ui(frame);
            // ui_style_text(frame);
            ui_scrollable_paragraph(frame);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')
                    || key.code == KeyCode::Char('Q')
                {
                    break;
                }
            }
        }
    }

    crossterm::execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}

fn main() -> Result<()> {
    hello_app()?;
    Ok(())
}
