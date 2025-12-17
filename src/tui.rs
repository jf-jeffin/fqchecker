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
    symbols::border,
    widgets::{self, Block, BorderType, Borders, Padding, Paragraph, Widget},
};
use tokio::sync::watch::Receiver;

#[derive(Debug, Default, Clone)]
pub struct Interface {
    pub info: CrrFileProcessInfo,
    pub exit: bool,
    // pub Init_time
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
            
            // if !self.info.is_file_reading{
            
                
                
            // }

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
        let block = Block::bordered()
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .title_top(Line::from("[ FQCHECK ]".bold()).left_aligned())
            .padding(Padding::new(1, 1, 1, 1))
            .border_type(BorderType::Thick);
        let inr = block.inner(area);
        block.render(area, buf);

        let main_layout= Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(inr);

        let l1 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25),Constraint::Percentage(75)])
            .split(main_layout[0]);

        let fgrrind = Block::bordered()
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .title_top(Line::from("[ INFO ]".bold()).left_aligned())
            .padding(Padding::uniform(1))
            .border_type(BorderType::Plain);
        fgrrind.clone().render(l1[0], buf);
        // let l1_layouts = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints([100])
        //     .split(l1[1]);

        let fgrrind_col2 = Block::bordered()
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .title_top(Line::from("[ INFO 2 ]".bold()).left_aligned())
            .padding(Padding::uniform(1))
            .border_type(BorderType::Plain);

        fgrrind_col2.render(l1[1], buf);
        // let t = k.std::time::Duration::abs_diff( )
        

        if self.info.is_file_reading {
            Paragraph::new(format!(
            "Processing...\nFile Name:{}\nSize: {}B\nRead Count: {}\nBase Count:{}",
            self.info.file_name,
            self.info.file_size,
            self.info.read_count,
            self.info.base_count))
            .block(Block::new())
            .render(fgrrind.inner(l1[0]), buf);
            return;
        }
        
    
        Paragraph::new(format!(
            "Processed\nFile Name: {}\nSize: {}B\nRead Count: {}\nBase Count:{}",
            self.info.file_name,
            self.info.file_size,
            self.info.read_count,
            self.info.base_count))
            .block(Block::new())
            .render(fgrrind.inner(l1[0]), buf);

         Block::bordered()
            .title_top(Line::from("[ Completed ]".bold()).left_aligned())
            .border_type(BorderType::Plain)
            .render(main_layout[1], buf);
    


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
