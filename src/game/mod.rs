pub mod snake;

extern crate termion;

use termion::input::{TermRead};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use self::termion::event::Key;
use std::{thread, time};
use std::sync::mpsc::{channel, TryRecvError};
use std::borrow::BorrowMut;
use rand::rngs::ThreadRng;
use rand::Rng;
use crate::game::snake::Point;
use self::termion::terminal_size;

pub fn main() {
    let mut stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let (term_width, term_height) = terminal_size().unwrap();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    let mut snake = snake::Snake::new(1, 1, term_width, term_height);
    snake.walk(snake::WalkDirection::Right, true);
    snake.walk(snake::WalkDirection::Right, true);
    snake.walk(snake::WalkDirection::Right, true);
    snake.walk(snake::WalkDirection::Right, true);
    snake.walk(snake::WalkDirection::Right, true);

    let (tx, rx) = channel();

    thread::spawn(move || {
        loop {
            for key in stdin.borrow_mut().keys() {
                tx.send(key.unwrap()).unwrap();
            }
        }
    });

    // Utils
    let rng = rand::thread_rng();

    // Game Logic
    let mut lose: bool;
    let mut last_move = Key::Right;
    let mut food = snake::Point::new(10, 10);

    loop {
        match rx.try_recv() {
            Ok(evt) => {
                match evt {
                    Key::Esc | Key::Ctrl('c') => {
                        break;
                    }
                    _ => {
                        let (new_snake, grow, status_lose) = move_snake(snake, evt, &food);

                        snake = new_snake;

                        if grow
                        {
                            food = generate_new_food(rng, term_width, term_height);
                        }

                        last_move = evt;

                        lose = status_lose;
                    }
                }
            }
            Err(TryRecvError::Empty) => {
                let (new_snake, grow, status_lose) = move_snake(snake, last_move, &food);

                snake = new_snake;

                if grow
                {
                    food = generate_new_food(rng, term_width, term_height);
                }

                lose = status_lose;
            }
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        if lose {
            break;
        }
        // Game speed
        thread::sleep(time::Duration::from_millis(100));

        // Render
        // Clear All
        write!(stdout, "{}", termion::clear::All).unwrap();

        // Food
        write!(
            stdout, "{}X",
            termion::cursor::Goto(food.x, food.y)
        ).unwrap();

        // Snake body
        for point in snake.body.iter() {
            write!(
                stdout, "{}o",
                termion::cursor::Goto(point.x, point.y)
            ).unwrap();
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}{}Pontuacao: {}{}",
           termion::cursor::Goto(1, 1),
           termion::clear::All,
           snake.body.len(),
           termion::cursor::Show
    ).unwrap();
}

fn generate_new_food(mut rng: ThreadRng, width: u16, height: u16) -> Point {
    snake::Point::new(
        rng.gen_range(0, width),
        rng.gen_range(0, height),
    )
}

fn move_snake(mut snake: snake::Snake, key: Key, food: &snake::Point) -> (snake::Snake, bool, bool) {
    let can_grow = snake.body.front().unwrap() == food;

    let lose = match key {
        Key::Up | Key::Char('w') => {
            snake.walk(snake::WalkDirection::Up, can_grow)
        }
        Key::Down | Key::Char('s') => {
            snake.walk(snake::WalkDirection::Down, can_grow)
        }
        Key::Left | Key::Char('a') => {
            snake.walk(snake::WalkDirection::Left, can_grow)
        }
        Key::Right | Key::Char('d') => {
            snake.walk(snake::WalkDirection::Right, can_grow)
        }
        _ => (true),
    };
    (snake, can_grow, !lose)
}