// use std::default::Default;
use std::sync::{Mutex, MutexGuard};
// use crossterm::style::Stylize;
use tokio::sync::{watch, watch::Receiver};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    Frame,
    buffer::Buffer,
    style::{ Style , Color,Stylize},
    text::{ Text,Line},
    // symbols::border,
    widgets::{
        self,
        Widget,
        BorderType,
        Paragraph,Block, Borders,
        Padding
    },
    layout::{
        Rect,
        Constraint, Layout, Direction
    },


};

// use ratatui::widgets::;
use crate::fqchecker::fqc::CrrFileProcessInfo;



#[derive(Debug, Default, Clone)]
pub struct Interface {
    pub info: CrrFileProcessInfo,
    // pub read_count: usize,
    // pub base_count: usize,
    // pub file_name: String,
    // pub directory_name: String,
    // pub info_chan : Receiver<CrrFileProcessInfo>

    pub exit: bool,
}





impl Interface {
    pub fn set_file_name(mut self,f: &str) -> Self {
        self.info.file_name = f.to_string();
        self
    }
    pub fn set_infos(mut self, new_value: CrrFileProcessInfo)  {
        self.info = new_value;
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal,rp: &mut  Receiver<CrrFileProcessInfo>) -> std::io::Result<()> {
        while !self.exit {
            match rp.borrow_and_update(){
                    info => { self.info = info.clone();}
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
        let area = frame.area();
        frame.render_widget(self,frame.area());

    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                // handle key
            }
        }
        Ok(())
    }

    // fn receive_status(self, rp: Receiver<CrrFileProcessInfo>){
    //
    // }


    fn exit(&mut self) {
        self.exit = true;
    }

}
impl Widget for &Interface {
    fn render(self,area: Rect, buf: &mut Buffer) {
        let title_str = Line::from("FQCHECK".bold()).left_aligned();
        // let title_style = Style::default().bg(Color::Blue).fg(Color::Black);
        let block = Block::bordered()
            .style(
                Style::default()
                    .bg(Color::Black)
                    .fg(Color::White))
            .title_top(Line::from("[ FQCHECK ]".bold()).left_aligned())
            .padding(Padding::new(1,1,1,1))
            .border_type(BorderType::Plain);
        let inr = block.inner(area);
        block.render(area, buf);

        let l1 = Layout::default().direction(Direction::Vertical).constraints(vec![15,75]).split(inr);
        let fgrrind = Block::bordered()
            .style(
                Style::default()
                    .bg(Color::Black)
                    .fg(Color::White))
            .title_top(Line::from("[ INFO ]".bold()).left_aligned())
            .padding(Padding::uniform(1))
            .border_type(BorderType::Plain);
        fgrrind.clone().render(l1[0], buf);
        let l1_layouts = Layout::default().direction(Direction::Vertical).constraints([50,50]).split(fgrrind.inner(l1[0]));
        let l1_layouts_c1 = Layout::default().direction(Direction::Horizontal).constraints([50,50]).split(l1_layouts[0]);
        let l1_layouts_c2 = Layout::default().direction(Direction::Horizontal).constraints([50,50]).split(l1_layouts[1]);

        Paragraph::new(self.info.read_count.to_string()).block(Block::new()).render(l1_layouts_c1[0], buf);
        Paragraph::new(self.info.file_name.clone()).block(Block::new()).render(l1_layouts_c1[1], buf);
        // Paragraph::new(self.info.file_name.clone()).block(fgrrind.clone()).render(l1_layouts_c2[0], buf);
        // Paragraph::new(self.info.file_name.clone()).block(fgrrind.clone()).render(l1_layouts_c2[1], buf);



        // Paragraph::new().block(block).render(area, buf);
        // let inner_str =



    }

}
use  std::sync::Arc;
use std::time::Duration;
use color_eyre::owo_colors::colors::Default;
use color_eyre::owo_colors::colors::xterm::SeaPink;
use crossterm::terminal::enable_raw_mode;
// use tokio::sync::watch::Receiver;

pub fn exec(app:&mut Interface, info_rec_chan : Receiver<CrrFileProcessInfo>) -> std::io::Result<()> {

     // let mut app =  app_state.lock().unwrap();
    enable_raw_mode()?;
    let mut tui = ratatui::init();
    let mut ih = info_rec_chan;
    let app = app.run(&mut tui, &mut ih);
    ratatui::restore();
    app
}

    //  {
    //     Ok(mut app) => {
    //         let mut tui = ratatui::init();
    //         let app = app.run(&mut tui);
    //         ratatui::restore();
    //         app
    //     },
    //     Err(e) => {Ok(())},
    // };

    // Ok(())
