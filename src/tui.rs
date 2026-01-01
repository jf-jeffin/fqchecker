use crate::{fqchecker::fqc::{BaseProfile, CrrFileProcessInfo, FQCOutput, FqcOpt, Helpers, ProcessingState}, main};
use color_eyre::owo_colors::OwoColorize;
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
    widgets::{self, Block, BorderType, Table,Row,Tabs, Padding, Paragraph, Widget},
};
use tabled::settings::Width;
use tokio::sync::watch::Receiver;


#[derive(Debug, Default, Clone)]
pub struct Interface {
    pub info: CrrFileProcessInfo,
    pub exit: bool,
    pub active_tab: usize,
    pub bp: BaseProfile,
    pub process_state: ProcessingState 
    // pub Init_time
}




impl Interface {
    pub fn set_file_name(mut self, f: &str) -> Self {
        self.info.file_name = f.to_string();
        self
    }   

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rp: &mut Receiver<CrrFileProcessInfo>,
        fqc_res: &FqcOpt
    ) -> std::io::Result<()> {
        
        
        while !self.exit {
            match rp.borrow_and_update() {
                info => {
                    self.info = info.clone();
                }
            
            }

// tokio::runtime::Handle::current().block_on(self.set_infos(fqc_res));
        // async {
            match self.info.is_file_reading {
                ProcessingState::Processed => { 
                    if let Ok(r) = fqc_res.read(){
                     self.bp = r.bp;
                }
            },
                _ => ()
            }
        // };
            // terminal.clear()?;
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            match self.handle_events() {
                Ok(_) => {},
                Err(e) => eprint!("Error in handling key input.{}",e)
            };
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
                    KeyCode::Char('1')  => self.set_active_tab(0),
                    KeyCode::Char('2') => self.set_active_tab(1),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn set_active_tab(&mut self,tab_index:usize){
        if tab_index <= 1 {
            self.active_tab = tab_index
        }
    }

    fn get_active_tab(&self) -> usize {
        if self.active_tab == 0 {
            self.active_tab + 1 
        }else {
            self.active_tab
        }
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
            .padding(Padding::top(1))
            .border_type(BorderType::Plain);
        fgrrind.clone().render(l1[0], buf);
        // let l1_layouts = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints([100])
        //     .split(l1[1]);

        let fgrrind_col2 = Block::bordered()
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .title_top(Line::from("[ INFO 2 ]".bold()).left_aligned())
            // .padding(Padding::uniform(1))
            .border_type(BorderType::Plain);
        
        fgrrind_col2.clone().render(l1[1], buf);

        let result_block = Block::bordered().style(Style::default().bg(Color::Black).fg(Color::White))
            .title_top(Line::from("[ Result ]".bold()).left_aligned())
            .padding(Padding::uniform(1))
            .border_type(BorderType::Plain);


        let id:String =  if let Some(l) = self.info.header.split_once(":"){
                // l.0.to_string().strip_prefix("@").unwrap().to_string();
                format!("Machine Id: {}\n", l.0.to_string().strip_prefix("@").unwrap().to_string())
            } else { 
                " ".to_string()
            } ; 
            Paragraph::new(id).render(fgrrind_col2.inner(l1[1]), buf);

        // fgrrind_col2.clone().render(l1[1], buf);
        // let t = k.std::time::Duration::abs_diff( )
        

            // upper_col2.clone().render(main_layout[1], buf);
        if self.info.is_file_reading == ProcessingState::Processing {
            Paragraph::new(format!(
            " Processing...\n\n File Name:{}\n Size: {}B\n Read Count: {}\n Base Count:{}",
            self.info.file_name,
            self.info.file_size,
            self.info.read_count,
            self.info.base_count))
            .block(Block::new())
            .render(fgrrind.inner(l1[0]), buf);
            return;
        }
        
    
        Paragraph::new(format!(
            "Processed\n\n File Name: {}\n Size: {}B\n Read Count: {}\n Base Count:{}",
            self.info.file_name,
            self.info.file_size,
            self.info.read_count,
            self.info.base_count))
            .block(Block::new())
            .render(fgrrind.inner(l1[0]), buf);

        // result_block.clone().render(main_layout[1], buf);
    //     Tabs::new(vec![" Baselevel ", " Readlevel "])
    // // .block(Block::bordered().title("Tabs"))
    // .style(Style::default().white())
    // .highlight_style(Style::default().green())
    // .select(self.active_tab)
    // .divider(" ")
    // .block(result_block)
    // .padding("|", "|").render(main_layout[1], buf);
    let result_str = format!("Total Bases: {}\nGC content: {}\nAmbigious Bases: {}", 
                    self.bp.total, 
                    self.bp.c+self.bp.g, 
                    self.bp.n);

        Paragraph::new(Text::from(result_str)).block(result_block).render(main_layout[1], buf);
        

    }
}

use std::time::Duration;

pub fn exec(
    app: &mut Interface,
    info_rec_chan: Receiver<CrrFileProcessInfo>,
    fqc_res: &mut FqcOpt
) -> std::io::Result<()> {
    let mut tui = ratatui::init();
    let mut ih = info_rec_chan;
    let app = app.run(&mut tui, &mut ih, &fqc_res);
    ratatui::restore();
    app
}
