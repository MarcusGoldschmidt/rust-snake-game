pub mod snake;

use crossterm::terminal::{self, Clear, ClearType};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, style,
};

use rand::rngs::ThreadRng;
use rand::Rng;
use std::io::{stdout, Write};
use std::sync::mpsc::{channel, TryRecvError};
use std::{thread, time};

use crate::game::snake::Point;

pub fn main() {
    let mut stdout = stdout();

    terminal::enable_raw_mode().unwrap();

    let (term_width, term_height) = terminal::size().unwrap();

    execute!(stdout, Clear(ClearType::All), cursor::Hide).unwrap();

    let mut snake = snake::Snake::new(1, 1, term_width, term_height);
    snake.walk(snake::WalkDirection::Right, true);
    snake.walk(snake::WalkDirection::Right, true);
    snake.walk(snake::WalkDirection::Right, true);
    snake.walk(snake::WalkDirection::Right, true);
    snake.walk(snake::WalkDirection::Right, true);

    let (tx, rx) = channel();

    thread::spawn(move || loop {
        loop {
            if let Event::Key(key_event) = event::read().unwrap() {
                tx.send(key_event).unwrap();
            }
        }
    });

    // Utils
    let rng = rand::thread_rng();

    // Game Logic
    let mut lose: bool;
    let mut last_move = KeyCode::Right;
    let mut food = snake::Point::new(10, 10);

    loop {
        match rx.try_recv() {
            Ok(key_event) => match key_event {
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                } => {
                    break;
                }
                KeyEvent { code, .. } => {
                    let (new_snake, grow, status_lose) = move_snake(snake, code, &food);

                    snake = new_snake;

                    if grow {
                        food = generate_new_food(rng, term_width, term_height);
                    }

                    last_move = code;

                    lose = status_lose;
                }
            },
            Err(TryRecvError::Empty) => {
                let (new_snake, grow, status_lose) = move_snake(snake, last_move, &food);

                snake = new_snake;

                if grow {
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
        queue!(stdout, Clear(ClearType::All)).unwrap();

        // Food
        queue!(stdout, cursor::MoveTo(food.x, food.y), style::Print("x")).unwrap();

        // Snake body
        for point in snake.body.iter() {
            queue!(stdout, cursor::MoveTo(point.x, point.y), style::Print("o")).unwrap();
        }

        stdout.flush().unwrap();
    }

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(1, 1),
        style::Print(format!("Pontuação: {}\n", snake.body.len()))
    )
    .unwrap();
}

fn generate_new_food(mut rng: ThreadRng, width: u16, height: u16) -> Point {
    snake::Point::new(rng.gen_range(0, width), rng.gen_range(0, height))
}

fn move_snake(
    mut snake: snake::Snake,
    key: KeyCode,
    food: &snake::Point,
) -> (snake::Snake, bool, bool) {
    let can_grow = snake.body.front().unwrap() == food;

    let lose = match key {
        KeyCode::Up | KeyCode::Char('w') => snake.walk(snake::WalkDirection::Up, can_grow),
        KeyCode::Down | KeyCode::Char('s') => snake.walk(snake::WalkDirection::Down, can_grow),
        KeyCode::Left | KeyCode::Char('a') => snake.walk(snake::WalkDirection::Left, can_grow),
        KeyCode::Right | KeyCode::Char('d') => snake.walk(snake::WalkDirection::Right, can_grow),
        _ => (true),
    };
    (snake, can_grow, !lose)
}
