use color_eyre::Result;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::*;
use ratatui::widgets::Paragraph;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                _ => (),
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let fret = " ---|";
    let frets = 22;
    let strings = 6;
    let fretboard = Paragraph::new(
        (0..strings)
            .map(|_| (0..frets).map(|_| fret.to_owned()).collect::<String>() + "\n")
            .collect::<String>(),
    );

    let area = center(
        frame.area(),
        Constraint::Length(frets * fret.len() as u16),
        Constraint::Length(strings),
    );

    frame.render_widget(fretboard, area);
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
