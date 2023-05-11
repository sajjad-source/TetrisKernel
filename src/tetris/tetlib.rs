use crate::keyboard::getch;
use crate::tetris::game::{HEIGHT, WIDTH};
use crate::tetris::gamescore::GameScore;
use crate::tetris::tetrominoe::{Tetrominoe, State, TColor};
use crate::vga_buffer::{change_color, Color, WRITER, clear_screen};
use crate::{print, dprint};

pub const EMP: char = '.';

pub fn render(
    display: &[[Tetrominoe; WIDTH]; HEIGHT],
    is_updated: bool,
    score: &mut GameScore,
    hold_piece: &Option<Tetrominoe>,
    next_piece: &Tetrominoe,
) {
    if !is_updated {
        return;
    }

    WRITER.lock().move_to(WIDTH + 3, 1).unwrap(); // move cursor to top left
    for (c, row) in display.iter().enumerate() {
        for ch in row {
            match ch.game_state {
                State::Empty => {
                    print!(" .");
                }
                State::Active | State::Landed => {
                    change_color(ch.as_color());
                    dprint!(0xdb); // print block
                    dprint!(0xdb);
                    change_color(Color::White);
                }
                State::Ghost => {
                    change_color(Color::DarkGray);
                    dprint!(0xb0); // print faded block
                    dprint!(0xb0);
                    change_color(Color::White);
                }
            }
        }
        WRITER.lock().move_to(WIDTH + 3, c + 2).unwrap();
    }

    // hold piece
    WRITER.lock().move_to(2, 1).unwrap();
    print!("Hold:");
    WRITER.lock().move_to(2, 3).unwrap();
    match hold_piece {
        Some(piece) => {
            let mut blank = Tetrominoe::new(None, None);
            let upright = blank.set(piece.ptype);
            for row in 0..upright.shape.len() {
                for col in 0..upright.shape[row].len() {
                    if upright.shape[row][col] == 'a' {
                        change_color(upright.as_color());
                        dprint!(0xdb); // print faded block
                        dprint!(0xdb);
                        change_color(Color::White);
                    } else {
                        print!("  ")
                    }
                }
                WRITER.lock().move_to(2, row + 4).unwrap();
            }
        }

        None => (),
    }

    // print stats
    WRITER.lock().move_to(WIDTH * 4, 1).unwrap();
    print!("Score: {}", score.score);
    WRITER.lock().move_to(WIDTH * 4, 3).unwrap();
    print!("Level: {}", score.level);
    WRITER.lock().move_to(WIDTH * 4, 5).unwrap();
    let time = score.get_time();
    print!("Time: {}:{:02}", time/60, time%60);

    // next piece
    WRITER.lock().move_to(WIDTH * 4, 8).unwrap();
    print!("Next:");
    WRITER.lock().move_to(WIDTH * 4, 10).unwrap();
    for row in 0..next_piece.shape.len() {
        for col in 0..next_piece.shape[row].len() {
            if next_piece.shape[row][col] == 'a' {
                change_color(next_piece.as_color());
                dprint!(0xdb); // print faded block
                dprint!(0xdb);
                change_color(Color::White);
            } else {
                print!("  ");
            }
        }
        WRITER.lock().move_to(WIDTH * 4, row + 11).unwrap();
    }

    WRITER.lock().flush();
}

pub fn init() -> [[Tetrominoe; WIDTH]; HEIGHT] {
    let display: [[Tetrominoe; WIDTH]; HEIGHT] = [[Tetrominoe::new(None, None); WIDTH]; HEIGHT];

    // walls
    clear_screen();
    WRITER.lock().move_to(11, 1).unwrap(); // move cursor to top left while leaving space for hold
    for row in display.iter().enumerate() {
        print!("<!"); // left wall
        for _ in row.1 {
            print!("  ");
        }
        print!("!>"); // right wall
        WRITER.lock().move_to(11, row.0 + 2).unwrap();
    }
    
    print!("<!===================!>\r\n"); // bottom wall
    print!("             \\/\\/\\/\\/\\/\\/\\/\\/\\/\\/",); // bottom spikes
    
    WRITER.lock().flush();

    display
}

pub fn gravity(
    display: &mut [[Tetrominoe; WIDTH]; HEIGHT],
    active_piece: &mut Tetrominoe,
    next_piece: &mut Tetrominoe,
) -> bool {
    let prev_display = display.clone();
    for row in (0..display.len()).rev() {
        for col in 0..display[row].len() {
            if display[row][col].game_state == State::Active {
                if row == display.len() - 1 || display[row + 1][col].game_state == State::Landed {
                    *display = prev_display;
                    landed(display);
                    let game_over = new_piece(display, active_piece, None, next_piece);
                    return game_over;
                }

                display[row + 1][col] = display[row][col];
                display[row][col] = Tetrominoe::new(None, None);
            }
        }
    }
    active_piece.row += 1;
    false
}

pub fn handle_input(
    display: &mut [[Tetrominoe; WIDTH]; HEIGHT],
    key: char,
    active_piece: &mut Tetrominoe,
    next_piece: &mut Tetrominoe,
) {
    let prev_display = display.clone();
    match key {
        'l' => {
            for row in (0..display.len()).rev() {
                for col in 0..display[row].len() {
                    if display[row][col].game_state == State::Active {
                        if col == 0 || display[row][col - 1].game_state == State::Landed {
                            *display = prev_display;
                            return;
                        }
                        display[row][col - 1] = display[row][col];
                        display[row][col] = Tetrominoe::new(None, None);
                    }
                }
            }

            if active_piece.col > 0 {
                active_piece.col -= 1;
            }
        }

        'r' => {
            for row in (0..display.len()).rev() {
                for col in (0..display[row].len()).rev() {
                    if display[row][col].game_state == State::Active {
                        if col == display[row].len() - 1 || display[row][col + 1].game_state == State::Landed {
                            *display = prev_display;
                            return;
                        }
                        display[row][col + 1] = display[row][col];
                        display[row][col] = Tetrominoe::new(None, None);
                    }
                }
            }
            active_piece.col += 1;
        }

        's' => {
            // bring down piece until new piece is created
            while display[0][display[0].len() / 2].game_state == State::Empty {
                gravity(display, active_piece, next_piece);
            }
        }

        'd' => {
            gravity(display, active_piece, next_piece);
        }

        'u' => {
            // let prev_display = display.clone();
            let prev_piece = active_piece.clone();

            // rotate piece
            active_piece.rotate();
            if active_piece.row + 4 > display.len() {
                active_piece.row = display.len() - 4;
            }

            if active_piece.col + 4 > display[0].len() {
                active_piece.col = display[0].len() - 4;
            }

            // clear piece and replace with new rotated piece
            for row in 0..display.len() {
                for col in 0..display[row].len() {
                    if display[row][col].game_state == State::Active {
                        display[row][col] = Tetrominoe::new(None, None);
                    }
                }
            }

            for row in active_piece.row..active_piece.row + 4 {
                for col in active_piece.col..active_piece.col + 4 {
                    if display[row][col].game_state == State::Landed {
                        *display = prev_display;
                        *active_piece = prev_piece;
                        return;
                    }

                    if active_piece.shape[row - active_piece.row][col - active_piece.col] == 'a' {
                        let new = active_piece.shape[row - active_piece.row][col - active_piece.col];
                        match new {
                            EMP => display[row][col] = Tetrominoe::new(None, None),
                            'a' => display[row][col] = Tetrominoe::new(Some(State::Active), Some(active_piece.color)),
                            _ => panic!("Invalid character in rotation matrix")
                        }
                    }
                }
            }
        }

        _ => (),
    }
}

pub fn new_piece(
    display: &mut [[Tetrominoe; WIDTH]; HEIGHT],
    active_piece: &mut Tetrominoe,
    desired_piece: Option<char>,
    next_piece: &mut Tetrominoe,
) -> bool {
    let half_width = display[0].len() / 2;

    // game over
    if display[0][half_width].game_state != State::Empty {
        return true;
    }

    let piece = desired_piece.unwrap_or_else(|| get_next_piece(next_piece));
    match piece {
        'I' => {
            // I
            // I
            // I
            // I
            display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
            display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
            display[2][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
            display[3][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
        }
        'J' => {
            //  J
            //  J
            // JJ
            display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
            display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
            display[2][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
            display[2][half_width - 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
        }
        'L' => {
            // L
            // L
            // LL
            display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
            display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
            display[2][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
            display[2][half_width + 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
        }
        'O' => {
            // OO
            // OO
            display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
            display[0][half_width + 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
            display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
            display[1][half_width + 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
        }
        'S' => {
            // SS
            //  SS
            display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Green));
            display[0][half_width + 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Green));
            display[1][half_width - 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Green));
            display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Green));
        }
        'T' => {
            // T
            // TT
            // T
            display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
            display[1][half_width - 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
            display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
            display[1][half_width + 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
        }
        'Z' => {
            //  ZZ
            // ZZ
            display[0][half_width - 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
            display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
            display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
            display[1][half_width + 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
        }
        _ => panic!("unknown picece: {}", piece),
    }
    active_piece.set(piece);
    active_piece.set_pos(0, half_width - 1);
    false
}

pub fn landed(display: &mut [[Tetrominoe; WIDTH]; HEIGHT]) {
    for row in display {
        for ch in row {
            if ch.game_state == State::Active {
                *ch = Tetrominoe::new(Some(State::Landed), Some(ch.color));
            }
        }
    }
}

pub fn full_line(display: &mut [[Tetrominoe; WIDTH]; HEIGHT], score: &mut GameScore) {
    let mut lines: usize = 0;
    'outer: for row in (0..display.len()).rev() {
        for ch in &display[row] {
            if ch.game_state != State::Landed {
                continue 'outer;
            }
        }
        display.remove(row);
        lines += 1;
    }

    for _ in 0..lines {
        match display.insert(0, [Tetrominoe::new(None, None); WIDTH]) { // add new line at the top
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        }
    }

    match lines {
        1 => score.score += 40 * (score.level + 1),
        2 => score.score += 100 * (score.level + 1),
        3 => score.score += 300 * (score.level + 1),
        4 => score.score += 1200 * (score.level + 1),
        _ => (),
    }

    score.level = score.score / 1000;
}

pub fn ghost_piece(display: &mut [[Tetrominoe; WIDTH]; HEIGHT], active_piece: &mut Tetrominoe) {
    for row in 0..display.len() {
        for col in 0..display[row].len() {
            if display[row][col].game_state == State::Ghost {
                display[row][col] = Tetrominoe::new(None, None);
            }
        }
    }

    let mut ghost = display.clone();
    let mut active_piece = active_piece.clone();

    gravity_until_new_piece(&mut ghost, &mut active_piece);

    for row in 0..ghost.len() {
        for col in 0..ghost[row].len() {
            if ghost[row][col].game_state == State::Active && display[row][col].game_state == State::Empty {
                display[row][col] = Tetrominoe::new(Some(State::Ghost), Some(ghost[row][col].color));
            }
        }
    }
}

fn gravity_until_new_piece(display: &mut [[Tetrominoe; WIDTH]; HEIGHT], active_piece: &mut Tetrominoe) {
    let mut prev_display = display.clone();
    gravity(
        display,
        active_piece,
        &mut Tetrominoe::random(),
    );
    while display[0][display[0].len() / 2].game_state == State::Empty {
        prev_display = display.clone();
        gravity(
            display,
            active_piece,
            &mut Tetrominoe::random(),
        );
    }
    *display = prev_display;
}

pub fn get_input(mut prev_scancode: &mut u8) -> char {
    if let Some(key) = getch(&mut prev_scancode) {
        match key {
            'q' => return 'q', // quit
            ' ' => return 's', // hard drop
            'c' => return 'c', // hold
            'p' => return 'p', // pause
            'i' | '8' => return 'u', // rotate clockwise (not sure why arrow keys are numbers)
            'k' | '2' => return 'd', // soft drop
            'j' | '4' => return 'l', // move left
            'l' | '6' => return 'r', // move right
            _ => return ' ',
        }
    } else {
        return ' ';
    }
}

pub fn hold(
    display: &mut [[Tetrominoe; WIDTH]; HEIGHT],
    active_piece: &mut Tetrominoe,
    hold_piece: &mut Option<Tetrominoe>,
    next_piece: &mut Tetrominoe,
) {
    // clear piece
    for row in display.iter_mut() {
        for col in row.iter_mut() {
            if col.game_state == State::Active {
                *col = Tetrominoe::new(None, None);
            }
        }
    }

    // hold piece
    if let Some(hold) = hold_piece {
        let prev_piece = active_piece.clone();
        new_piece(display, active_piece, Some(hold.ptype), next_piece);
        *hold_piece = Some(prev_piece);
    } else {
        *hold_piece = Some(active_piece.clone());
        new_piece(display, active_piece, None, next_piece);
    }
}

fn get_next_piece(next_piece: &mut Tetrominoe) -> char {
    let temp = next_piece.ptype;
    *next_piece = Tetrominoe::random();
    temp
}

pub fn put_text(text: &str) {
    let width = WIDTH * 2 - text.len() / 4;
    WRITER.lock().move_to(width, HEIGHT / 2).unwrap();
    change_color(Color::LightRed);
    print!("{}", text);
    change_color(Color::White);
}

trait Remove {
    fn remove(&mut self, index: usize) -> Self;
}

trait Insert<T> {
    fn insert(&mut self, index: usize, item: T) -> Result<(), &'static str>;
}

impl<T: Clone + Copy, const N: usize> Remove for [T; N] {
    fn remove(&mut self, index: usize) -> Self {
        let mut temp = self.clone();
        temp[index] = temp[N - 1];
        temp[N - 1] = self[index];
        temp
    }
}

impl<T, const N: usize> Insert<T> for [T; N]
where
    T: Copy + Default,
{
    fn insert(&mut self, index: usize, item: T) -> Result<(), &'static str> {
        if index > N {
            return Err("Index out of bounds");
        }

        if index < N - 1 {
            for i in (index + 1..N).rev() {
                self[i] = self[i - 1];
            }
        }

        self[index] = item;
        Ok(())
    }
}