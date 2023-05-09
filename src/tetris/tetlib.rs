use crate::keyboard::getch;
use crate::tetris::game::{HEIGHT, WIDTH};
use crate::tetris::gamescore::GameScore;
use crate::tetris::tetrominoe::Tetrominoe;
use crate::vga_buffer::clear_screen;
use crate::vga_buffer::{change_color, Color, WRITER};
use crate::{print, println};

pub const EMP: char = '.';

pub fn render(
    display: &[[char; WIDTH]; HEIGHT],
    is_updated: bool,
    score: &mut GameScore,
    hold_piece: &Option<Tetrominoe>,
    next_piece: &Tetrominoe,
) {
    if !is_updated {
        return;
    }

    if !is_updated {
        return;
    }

    clear_screen();
    let _width: u16 = display[0].len() as u16;

    for (_c, row) in display.iter().enumerate() {
        for ch in row {
            match ch {
                &EMP => {
                    print!(" .");
                }
                'a' => {
                    print!("[]");
                }
                'l' => {
                    print!("[]");
                }
                'g' => {
                    change_color(Color::DarkGray);
                    print!("//");
                    change_color(Color::White);
                }

                _ => panic!("unknown character: {}", ch),
            }
        }
        println!();
    }
    WRITER.lock().flush();
}

pub fn init() -> [[char; WIDTH]; HEIGHT] {
    let display: [[char; WIDTH]; HEIGHT] = [[EMP; WIDTH]; HEIGHT];
    display
}

pub fn gravity(
    display: &mut [[char; WIDTH]; HEIGHT],
    active_piece: &mut Tetrominoe,
    next_piece: &mut Tetrominoe,
) -> bool {
    let prev_display = display.clone();
    for row in (0..display.len()).rev() {
        for col in 0..display[row].len() {
            if display[row][col] == 'a' {
                if row == display.len() - 1 || display[row + 1][col] == 'l' {
                    *display = prev_display;
                    landed(display);
                    let game_over = new_piece(display, active_piece, None, next_piece);
                    return game_over;
                }

                display[row][col] = EMP;
                display[row + 1][col] = 'a';
            }
        }
    }
    active_piece.row += 1;
    false
}

pub fn handle_input(
    display: &mut [[char; WIDTH]; HEIGHT],
    key: char,
    active_piece: &mut Tetrominoe,
    next_piece: &mut Tetrominoe,
) {
    let prev_display = display.clone();
    match key {
        'l' => {
            for row in (0..display.len()).rev() {
                for col in 0..display[row].len() {
                    if display[row][col] == 'a' {
                        if col == 0 || display[row][col - 1] == 'l' {
                            *display = prev_display;
                            return;
                        }
                        display[row][col] = EMP;
                        display[row][col - 1] = 'a';
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
                    if display[row][col] == 'a' {
                        if col == display[row].len() - 1 || display[row][col + 1] == 'l' {
                            *display = prev_display;
                            return;
                        }
                        display[row][col] = EMP;
                        display[row][col + 1] = 'a';
                    }
                }
            }
            active_piece.col += 1;
        }

        's' => {
            // bring down piece until new piece is created
            while display[0][display[0].len() / 2] == EMP {
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
                    if display[row][col] == 'a' {
                        display[row][col] = EMP;
                    }
                }
            }

            for row in active_piece.row..active_piece.row + 4 {
                for col in active_piece.col..active_piece.col + 4 {
                    if display[row][col] == 'l' {
                        *display = prev_display;
                        *active_piece = prev_piece;
                        return;
                    }

                    if active_piece.shape[row - active_piece.row][col - active_piece.col] == 'a' {
                        display[row][col] =
                            active_piece.shape[row - active_piece.row][col - active_piece.col];
                    }
                }
            }
        }

        _ => (),
    }
}

pub fn new_piece(
    display: &mut [[char; WIDTH]; HEIGHT],
    active_piece: &mut Tetrominoe,
    desired_piece: Option<char>,
    next_piece: &mut Tetrominoe,
) -> bool {
    let half_width = display[0].len() / 2;

    // game over
    if display[0][half_width] != EMP {
        return true;
    }

    let piece = desired_piece.unwrap_or_else(|| get_next_piece(next_piece));
    match piece {
        'I' => {
            // I
            // I
            // I
            // I
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[2][half_width] = 'a';
            display[3][half_width] = 'a';
        }
        'J' => {
            //  J
            //  J
            // JJ
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[2][half_width] = 'a';
            display[2][half_width - 1] = 'a';
        }
        'L' => {
            // L
            // L
            // LL
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[2][half_width] = 'a';
            display[2][half_width + 1] = 'a';
        }
        'O' => {
            // OO
            // OO
            display[0][half_width] = 'a';
            display[0][half_width + 1] = 'a';
            display[1][half_width] = 'a';
            display[1][half_width + 1] = 'a';
        }
        'S' => {
            // SS
            //  SS
            display[0][half_width] = 'a';
            display[0][half_width + 1] = 'a';
            display[1][half_width - 1] = 'a';
            display[1][half_width] = 'a';
        }
        'T' => {
            // T
            // TT
            // T
            display[0][half_width] = 'a';
            display[1][half_width - 1] = 'a';
            display[1][half_width] = 'a';
            display[1][half_width + 1] = 'a';
        }
        'Z' => {
            //  ZZ
            // ZZ
            display[0][half_width - 1] = 'a';
            display[0][half_width] = 'a';
            display[1][half_width] = 'a';
            display[1][half_width + 1] = 'a';
        }
        _ => panic!("unknown picece: {}", piece),
    }
    active_piece.set(piece);
    active_piece.set_pos(0, half_width - 1);
    false
}

pub fn landed(display: &mut [[char; WIDTH]; HEIGHT]) {
    for row in display {
        for ch in row {
            if *ch == 'a' {
                *ch = 'l';
            }
        }
    }
}

pub fn full_line(display: &mut [[char; WIDTH]; HEIGHT], score: &mut GameScore) {
    let mut lines: usize = 0;
    'outer: for row in (0..display.len()).rev() {
        for ch in &display[row] {
            if *ch != 'l' {
                continue 'outer;
            }
        }
        display.remove(row);
        lines += 1;
    }

    for _ in 0..lines {
        display.insert(0, [EMP; WIDTH]); // add new line at the top
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

pub fn ghost_piece(display: &mut [[char; WIDTH]; HEIGHT], active_piece: &mut Tetrominoe) {
    for row in 0..display.len() {
        for col in 0..display[row].len() {
            if display[row][col] == 'g' {
                display[row][col] = EMP;
            }
        }
    }

    let mut ghost = display.clone();
    let mut active_piece = active_piece.clone();

    gravity_until_new_piece(&mut ghost, &mut active_piece);

    for row in 0..ghost.len() {
        for col in 0..ghost[row].len() {
            if ghost[row][col] == 'a' && display[row][col] == EMP {
                display[row][col] = 'g';
            }
        }
    }
}

fn gravity_until_new_piece(display: &mut [[char; WIDTH]; HEIGHT], active_piece: &mut Tetrominoe) {
    let mut prev_display = display.clone();
    gravity(
        display,
        active_piece,
        &mut Tetrominoe::random(2 + active_piece.col),
    );
    while display[0][display[0].len() / 2] == EMP {
        prev_display = display.clone();
        gravity(
            display,
            active_piece,
            &mut Tetrominoe::random(2 + active_piece.col),
        );
    }
    *display = prev_display;
}

pub fn get_input() -> char {
    if let Some(key) = getch(&mut 0u8) {
        match key {
            'q' => return 'q', // quit
            ' ' => return 's', // hard drop
            'c' => return 'c', // hold
            'p' => return 'p', // pause
            'i' => return 'u', // rotate clockwise
            'k' => return 'd', // soft drop
            'j' => return 'l', // move left
            'l' => return 'r', // move right
            _ => return ' ',
        }
    } else {
        return ' ';
    }
}

pub fn hold(
    display: &mut [[char; WIDTH]; HEIGHT],
    active_piece: &mut Tetrominoe,
    hold_piece: &mut Option<Tetrominoe>,
    next_piece: &mut Tetrominoe,
) {
    // clear piece
    for row in display.iter_mut() {
        for col in row.iter_mut() {
            if *col == 'a' {
                *col = EMP;
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
    *next_piece = Tetrominoe::random(next_piece.col + 4);
    temp
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
