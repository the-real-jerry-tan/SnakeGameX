
// main.rs
// Copyright (c) 2024 Your Name
// This code is licensed under the MIT License.

// This is the main entry point for the Snake Game with a Text User Interface (TUI).

use crossterm::{cursor, execute, terminal, ExecutableCommand, event::{self, Event, KeyCode}};
use std::io::{self, Write};
use std::time::{Duration, Instant};

const SNAKE_CHAR: char = 'O';
const EMPTY_CHAR: char = ' ';
const WIDTH: usize = 20;
const HEIGHT: usize = 10;

// Struct to represent the snake
struct Snake {
    body: Vec<(usize, usize)>, // Snake body is a vector of (x, y) coordinates
    direction: (isize, isize), // Direction the snake is moving (dx, dy)
}

impl Snake {
    fn new() -> Self {
        // Create a new snake with an initial position and direction
        Snake {
            body: vec![(WIDTH / 2, HEIGHT / 2)],
            direction: (1, 0), // Start moving to the right
        }
    }

    fn move_forward(&mut self) {
        let head = self.body[0];
        let new_head = (
            ((head.0 as isize + self.direction.0) as usize + WIDTH) % WIDTH,
            ((head.1 as isize + self.direction.1) as usize + HEIGHT) % HEIGHT,
        );
        self.body.insert(0, new_head); // Add new head to the snake's body
        self.body.pop(); // Remove the tail to simulate movement
    }

    fn change_direction(&mut self, direction: (isize, isize)) {
        self.direction = direction;
    }
}

fn draw(snake: &Snake) {
    // Clear the screen and move the cursor to the top left
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    // Draw the game field with the snake's position
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if snake.body.contains(&(x, y)) {
                print!("{}", SNAKE_CHAR);
            } else {
                print!("{}", EMPTY_CHAR);
            }
        }
        println!();
    }
    stdout.flush().unwrap();
}

fn main() {
    let mut snake = Snake::new();
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap(); // Enable raw mode for terminal input

    let mut last_instant = Instant::now();
    let speed = Duration::from_millis(200);

    // Game loop
    loop {
        // Handle input for snake movement
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => snake.change_direction((0, -1)),
                    KeyCode::Down => snake.change_direction((0, 1)),
                    KeyCode::Left => snake.change_direction((-1, 0)),
                    KeyCode::Right => snake.change_direction((1, 0)),
                    KeyCode::Char('q') => break, // Press 'q' to quit
                    _ => {}
                }
            }
        }

        // Move the snake forward at regular intervals
        if last_instant.elapsed() >= speed {
            snake.move_forward();
            draw(&snake); // Redraw the game with the updated snake position
            last_instant = Instant::now();
        }
    }

    // Restore terminal state
    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
}
