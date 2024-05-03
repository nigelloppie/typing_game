use std::{
    borrow::Borrow,
    io::{stdout, Result},
    str::Matches,
    u8,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use memorable_wordlist::WORDS;
use rand::{self, thread_rng, Rng};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
mod tui;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    word_list: Vec<String>,
    typed_letters: Vec<String>,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.word_list = get_word_list();
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
                self.handle_key_event(key_event)
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
            KeyCode::Char(c) => self.typed_letters.push(c.to_string()),
            KeyCode::Backspace => match self.typed_letters.pop() {
                Some(top) => {}
                None => {}
            },
            _ => {}
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

    fn check_correct_char(&self) -> Vec<Span> {
        let mut text: Vec<Span> = Vec::new();
        let words = self.word_list.join("");
        let words_display = self.word_list.join("");
        let typed = self.typed_letters.join("");
        let mut words_iter = words.chars();
        let mut typed_iter = typed.chars();
        let mut words_display_iter = words_display.chars();
        loop {
            match (
                words_iter.next(),
                typed_iter.next(),
                words_display_iter.next(),
            ) {
                (Some(word), Some(typed), Some(display)) => {
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
                        }
                    }
                }
                (Some(word), None, Some(display)) => text.push(Span::styled(
                    display.to_string(),
                    Style::default().fg(Color::DarkGray),
                )),
                (None, None, None) => {
                    break;
                }
                _ => {}
            }
        }
        text
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

        Paragraph::new(Line::from(text))
            .centered()
            .wrap(Wrap { trim: true })
            .block(block)
            .render(area, buf);
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

    for _n in 1..100 {
        let x: usize = rng.gen_range(0..WORDS.len());
        word_list.push(WORDS[x].to_owned() + " ");
    }

    word_list
}
