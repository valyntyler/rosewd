use color_eyre::Result;
use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use ratatui::Frame;
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
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let fretboard = Paragraph::new(
        (0..6)
            .map(|_| (0..22).map(|_| " ---|").collect::<String>() + "\n")
            .collect::<String>(),
    );

    frame.render_widget(fretboard, frame.area());
}
