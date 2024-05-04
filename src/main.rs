use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use memorable_wordlist::WORDS;
use rand::{self, Rng};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::time::SystemTime;
use std::{io::Result, time::UNIX_EPOCH, u64, u8};
mod tui;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    word_list: Vec<String>,
    typed_letters: Vec<String>,
    playing: bool,
    done: bool,
    start_time: u64,
    duration: String,
    correct_char: u64,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.word_list = get_word_list();
        self.correct_char = 0;
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_event()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_event(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
                self.timer();
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            KeyCode::Char(c) => {
                self.typed_letters.push(c.to_string());
            }
            KeyCode::Backspace => match self.typed_letters.pop() {
                Some(_top) => {}
                None => {}
            },
            _ => {}
        }
    }

    fn timer(&mut self) {
        if !self.playing {
            self.done = false;
            self.playing = true;
            self.start_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }

        if self.typed_letters.join("").len() >= self.word_list.join("").len() && !self.done {
            self.set_correct_char_count();
            self.playing = false;
            self.done = true;
            let end_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let start_time = UNIX_EPOCH + std::time::Duration::from_secs(self.start_time);
            let end_time = UNIX_EPOCH + std::time::Duration::from_secs(end_time);
            let duration = end_time
                .duration_since(start_time)
                .expect("Time went backwards");
            let total_secs = duration.as_secs();
            let minutes = total_secs / 60;
            let seconds = total_secs % 60;
            self.duration = format!("{:02}:{:02}", minutes, seconds);
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    pub fn set_done(&mut self) {
        self.done = true;
    }

    fn check_correct_char(&self) -> Vec<Span> {
        let mut text: Vec<Span> = Vec::new();
        let words = self.word_list.join("");
        let typed = self.typed_letters.join("");
        let mut words_iter = words.chars();
        let mut typed_iter = typed.chars();
        loop {
            match (words_iter.next(), typed_iter.next()) {
                (Some(word), Some(typed)) => {
                    if word == typed {
                        text.push(Span::styled(
                            typed.to_string(),
                            Style::default().fg(Color::LightYellow),
                        ))
                    } else {
                        if typed != ' ' {
                            text.push(Span::styled(
                                typed.to_string(),
                                Style::default().fg(Color::Red),
                            ))
                        } else {
                            text.push(Span::styled(
                                '_'.to_string(),
                                Style::default().fg(Color::Red),
                            ))
                        }
                    }
                }
                (Some(word), None) => text.push(Span::styled(
                    word.to_string(),
                    Style::default().fg(Color::DarkGray),
                )),
                (None, None) => {
                    break;
                }
                _ => {}
            }
        }
        text
    }

    fn set_correct_char_count(&mut self) {
        let words = self.word_list.join("");
        let typed = self.typed_letters.join("");
        let mut words_iter = words.chars();
        let mut typed_iter = typed.chars();
        loop {
            match (words_iter.next(), typed_iter.next()) {
                (Some(word), Some(typed)) => {
                    if word == typed {
                        self.correct_char += 1;
                    }
                }
                (None, None) => {
                    break;
                }
                _ => {}
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Terminal Typer ".bold());
        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<ESC> ".blue().bold()]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let text: Vec<Span> = App::check_correct_char(&self);
        if self.typed_letters.join("").len() >= self.word_list.join("").len() {
            Paragraph::new(format!(
                "You took: {}\nAccuracy: {}/{}",
                self.duration,
                self.correct_char,
                self.word_list.join("").len()
            ))
            .centered()
            .wrap(Wrap { trim: true })
            .block(block.padding(Padding::new(
                area.width / 4,
                area.width / 4,
                area.height / 4,
                0,
            )))
            .render(area, buf);
        } else {
            Paragraph::new(Line::from(text))
                .centered()
                .wrap(Wrap { trim: true })
                .block(block.padding(Padding::new(
                    area.width / 4,
                    area.width / 4,
                    area.height / 4,
                    0,
                )))
                .render(area, buf);
        }
    }
}

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let _app_result = App::default().run(&mut terminal);
    tui::restore()?;
    Ok(())
}

fn get_word_list() -> Vec<String> {
    let mut word_list = vec![];
    let mut rng = rand::thread_rng();

    for n in 1..10 {
        let x: usize = rng.gen_range(0..WORDS.len());
        if n == 9 {
            word_list.push(WORDS[x].to_owned());
        } else {
            word_list.push(WORDS[x].to_owned() + " ");
        }
    }

    word_list
}
