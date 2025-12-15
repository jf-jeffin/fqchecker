use crate::fqchecker::fqc::CrrFileProcessInfo;
use crossterm::event::{
    self,
    Event,
    KeyCode, //  KeyCode, KeyEvent, KeyEventKind
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{self, Block, BorderType, Borders, Padding, Paragraph, Widget},
};
use tokio::sync::watch::Receiver;

#[derive(Debug, Default, Clone)]
pub struct Interface {
    pub info: CrrFileProcessInfo,
    pub exit: bool,
}

impl Interface {
    pub fn set_file_name(mut self, f: &str) -> Self {
        self.info.file_name = f.to_string();
        self
    }
    pub fn set_infos(mut self, new_value: CrrFileProcessInfo) {
        self.info = new_value;
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rp: &mut Receiver<CrrFileProcessInfo>,
    ) -> std::io::Result<()> {
        while !self.exit {
            match rp.borrow_and_update() {
                info => {
                    self.info = info.clone();
                }
            }

            // terminal.clear()?;
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            self.handle_events();
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
impl Widget for &Interface {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_str = Line::from("FQCHECK".bold()).left_aligned();
        let block = Block::bordered()
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .title_top(Line::from("[ FQCHECK ]".bold()).left_aligned())
            .padding(Padding::new(1, 1, 1, 1))
            .border_type(BorderType::Plain);
        let inr = block.inner(area);
        block.render(area, buf);

        let l1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![15, 75])
            .split(inr);
        let fgrrind = Block::bordered()
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .title_top(Line::from("[ INFO ]".bold()).left_aligned())
            .padding(Padding::uniform(1))
            .border_type(BorderType::Plain);
        fgrrind.clone().render(l1[0], buf);
        let l1_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([50, 50])
            .split(fgrrind.inner(l1[0]));
        let l1_layouts_c1 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([50, 50])
            .split(l1_layouts[0]);
        let l1_layouts_c2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([50, 50])
            .split(l1_layouts[1]);

        Paragraph::new(self.info.read_count.to_string())
            .block(Block::new())
            .render(l1_layouts_c1[0], buf);
        Paragraph::new(self.info.file_name.clone())
            .block(Block::new())
            .render(l1_layouts_c1[1], buf);
    }
}

use std::time::Duration;

pub fn exec(
    app: &mut Interface,
    info_rec_chan: Receiver<CrrFileProcessInfo>,
) -> std::io::Result<()> {
    let mut tui = ratatui::init();
    let mut ih = info_rec_chan;
    let app = app.run(&mut tui, &mut ih);
    ratatui::restore();
    app
}
