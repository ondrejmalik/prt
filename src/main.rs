#![feature(duration_millis_float)]

use crate::game_sizing::{get_scale_x, get_scale_y, WINDOW_SIZE_BASE};
use raylib::prelude::*;
use std::time::{Duration, Instant};
use winapi::um::processthreadsapi::{
    GetCurrentProcess, GetCurrentThread, GetProcessPriorityBoost, GetThreadPriority,
    SetPriorityClass, SetThreadPriority,
};
use winapi::um::winbase::REALTIME_PRIORITY_CLASS;
use winapi::um::winbase::THREAD_PRIORITY_TIME_CRITICAL;
trait Drawable {
    fn draw(&self, d: &mut RaylibDrawHandle);
}

struct GameScore {
    score_blue: u8,
    score_red: u8,
}
impl Drawable for GameScore {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            format!("{} : {}", self.score_blue, self.score_red).as_str(),
            WINDOW_SIZE_BASE.0 / 2,
            20,
            50,
            Color::BLACK,
        );
    }
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}
impl Drawable for Circle {
    fn draw(&self, mut d: &mut RaylibDrawHandle) {
        d.draw_circle(
            (self.x * get_scale_x(&d)) as i32,
            (self.y * get_scale_y(&d)) as i32,
            self.radius * get_scale_x(&d),
            Color::GREEN,
        );
    }
}

impl Drawable for Rectangle {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            (self.x * get_scale_x(&d)) as i32,
            (self.y * get_scale_y(&d)) as i32,
            (self.width * get_scale_x(&d)) as i32,
            (self.height * get_scale_y(&d)) as i32,
            Color::BLUE,
        );
    }
}
enum GameState {
    Running(BallMovement),
    Stopped,
}
enum BallMovement {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
mod game_sizing {
    use raylib::drawing::RaylibDrawHandle;
    use raylib::RaylibHandle;

    pub const WINDOW_SIZE_BASE: (i32, i32) = (1280, 720);
    pub fn get_scale_x(rl: &RaylibDrawHandle) -> f32 {
        rl.get_screen_width() as f32 / WINDOW_SIZE_BASE.0 as f32
    }
    pub fn get_scale_y(rl: &RaylibHandle) -> f32 {
        rl.get_screen_height() as f32 / WINDOW_SIZE_BASE.1 as f32
    }
}
fn main() {
    unsafe {
        // Get the handle to the current process
        let process = GetCurrentProcess();
        let thread = GetCurrentThread();
        let priority = GetThreadPriority(thread);
        println!("prio: {}", priority);
        // Set the priority class to high
        let class = SetPriorityClass(process, REALTIME_PRIORITY_CLASS);
        let prio = SetThreadPriority(process, THREAD_PRIORITY_TIME_CRITICAL.try_into().unwrap());
        println!("Class: {}", class);
        println!("prio: {}", priority);
        let priority = GetThreadPriority(thread);
        println!("prio: {}", priority);
        // Set the thread priority to TIME_CRITICAL
        if SetThreadPriority(thread, THREAD_PRIORITY_TIME_CRITICAL.try_into().unwrap()) == 0 {
            eprintln!("Failed to set thread priority to TIME_CRITICAL");
        } else {
            println!("Thread priority set to TIME_CRITICAL successfully.");
        }
    }

    let mut game_state = GameState::Stopped;
    let mut game_score = GameScore {
        score_blue: 0,
        score_red: 0,
    };
    let mut player1 = Rectangle {
        x: 10.0,
        y: (WINDOW_SIZE_BASE.1 / 2 - 5) as f32,
        width: 5f32,
        height: 50f32,
    };
    let mut player2 = Rectangle {
        x: (WINDOW_SIZE_BASE.0 - 10) as f32,
        y: (WINDOW_SIZE_BASE.1 / 2 - 5) as f32,
        width: 5f32,
        height: 50f32,
    };

    let mut ball = Circle {
        x: (WINDOW_SIZE_BASE.0 / 2) as f32,
        y: (WINDOW_SIZE_BASE.1 / 2) as f32,
        radius: 10.0,
    };

    let audio = RaylibAudio::init_audio_device().unwrap();

    let sound = audio.new_sound("sounds/pog.wav").unwrap();
    sound.play();
    let (mut rl, thread) = init()
        .size(WINDOW_SIZE_BASE.0, WINDOW_SIZE_BASE.1)
        .resizable()
        .title("Raylib")
        .build();

    let time_to_next = Duration::from_millis(4);
    let mut current = Instant::now();
    //main loop
    while !rl.window_should_close() {
        // input
        input(
            &mut current,
            &time_to_next,
            &rl,
            &mut player1,
            &mut player2,
            &mut ball,
            &mut game_state,
            &mut game_score,
        );
        let score_text = format!("{} : {}", game_score.score_blue, game_score.score_red);
        let width = rl.measure_text(&score_text, 40);
        // draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        //draw players
        player1.draw(&mut d);
        player2.draw(&mut d);
        ball.draw(&mut d);
        d.draw_text(
            format!("FPS: {}", d.get_fps()).as_str(),
            12,
            20,
            20,
            Color::BLACK,
        );
        d.draw_text(
            score_text.as_str(),
            ((WINDOW_SIZE_BASE.0 as f32 * get_scale_x(&d)) as i32 / 2) - width / 2,
            20,
            40,
            Color::BLACK,
        );
    }
}
fn input(
    current: &mut Instant,
    time_to_next: &Duration,
    rl: &RaylibHandle,
    player1: &mut Rectangle,
    player2: &mut Rectangle,
    ball: &mut Circle,
    game_state: &mut GameState,
    game_score: &mut GameScore,
) {
    if current.elapsed() >= *time_to_next {
        let move_multiplier = current.elapsed().as_millis_f32() / time_to_next.as_millis_f32();
        //player 1
        if rl.is_key_down(key_from_i32(87).unwrap()) {
            if player1.y >= 0.0 {
                player1.y -= 2.0 * move_multiplier;
                change_game_state(game_state);
            }
        } else if rl.is_key_down(key_from_i32(83).unwrap()) {
            if (player1.y + player1.height) * get_scale_y(rl) <= rl.get_screen_height() as f32 {
                player1.y += 2.0 * move_multiplier;
                change_game_state(game_state);
            }
        }
        //player 2
        if rl.is_key_down(key_from_i32(265).unwrap()) {
            if player2.y >= 0.0 {
                player2.y -= 2.0 * move_multiplier;
                change_game_state(game_state);
            }
        } else if rl.is_key_down(key_from_i32(264).unwrap()) {
            if (player2.y + player2.height) * get_scale_y(rl) <= rl.get_screen_height() as f32 {
                player2.y += 2.0 * move_multiplier;
                change_game_state(game_state);
            }
        }

        move_ball(ball, game_state, rl, move_multiplier);
        //check left and right border
        if (ball.x as f32 + ball.radius) * get_scale_y(rl) > rl.get_screen_width() as f32 {
            game_score.score_blue += 1;
            *game_state = GameState::Stopped;
            ball_set_default_pos(ball);
        } else if ball.x - ball.radius <= 0f32 {
            game_score.score_red += 1;
            *game_state = GameState::Stopped;
            ball_set_default_pos(ball);
        }

        //check ball x player1 collision
        if ball.y + ball.radius >= player1.y
            && (ball.y - ball.radius) <= (player1.y + player1.height)
        {
            if ball.x <= player1.x + player1.width {
                match game_state {
                    GameState::Running(ball_movement) => match ball_movement {
                        BallMovement::TopLeft => *ball_movement = BallMovement::TopRight,
                        BallMovement::BottomLeft => *ball_movement = BallMovement::BottomRight,
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    },
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
        }
        //check ball x player2 collision
        if ball.y + ball.radius >= player2.y
            && (ball.y - ball.radius) <= (player2.y + player2.height)
        {
            if ball.x >= player2.x - player2.width {
                match game_state {
                    GameState::Running(ball_movement) => match ball_movement {
                        BallMovement::TopRight => *ball_movement = BallMovement::TopLeft,
                        BallMovement::BottomRight => *ball_movement = BallMovement::BottomLeft,
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    },
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
        }

        *current = Instant::now();
    }
    fn change_game_state(game_state: &mut GameState) {
        match game_state {
            GameState::Running(_) => {}
            GameState::Stopped => {
                *game_state = GameState::Running(BallMovement::TopLeft);
            }
        }
    }
    fn move_ball(
        ball: &mut Circle,
        game_state: &mut GameState,
        rl: &RaylibHandle,
        move_multiplier: f32,
    ) {
        match game_state {
            GameState::Running(ball_movement) => {
                //check top and bottom border
                if ball.y as f32 + ball.radius * get_scale_y(rl) > rl.get_screen_height() as f32
                    || ball.y as f32 - ball.radius < 0f32
                {
                    match ball_movement {
                        BallMovement::TopLeft => *ball_movement = BallMovement::BottomLeft,
                        BallMovement::TopRight => *ball_movement = BallMovement::BottomRight,
                        BallMovement::BottomLeft => *ball_movement = BallMovement::TopLeft,
                        BallMovement::BottomRight => *ball_movement = BallMovement::TopRight,
                    }
                }

                match ball_movement {
                    BallMovement::TopLeft => {
                        ball.x -= 1.0 * move_multiplier;
                        ball.y -= 1.0 * move_multiplier;
                    }
                    BallMovement::TopRight => {
                        ball.x += 1.0 * move_multiplier;
                        ball.y -= 1.0 * move_multiplier;
                    }
                    BallMovement::BottomLeft => {
                        ball.x -= 1.0 * move_multiplier;
                        ball.y += 1.0 * move_multiplier;
                    }
                    BallMovement::BottomRight => {
                        ball.x += 1.0 * move_multiplier;
                        ball.y += 1.0 * move_multiplier;
                    }
                }
            }
            GameState::Stopped => {}
        }
    }
}
fn ball_set_default_pos(ball: &mut Circle) {
    ball.x = (WINDOW_SIZE_BASE.0 / 2) as f32;
    ball.y = (WINDOW_SIZE_BASE.1 / 2) as f32;
}
