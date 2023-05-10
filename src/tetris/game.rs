// Tetris
use crate::tetris::gamestate::GameState;
use crate::tetris::tetlib::*;
use crate::vga_buffer::WRITER;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub fn run() {
    let mut grav_tick: usize = 100;

    let mut gs = GameState::new();
    let mut prev_scancode = 0; // required for key repeat

    // main loop
    loop {
        let prev_gs = gs.clone();

        // handle input
        let key = get_input(&mut prev_scancode);

        // quit
        if key == 'q' {
            break;
        }

        // gravity
        if gs.counter >= grav_tick {
            if gravity(&mut gs.display, &mut gs.active_piece, &mut gs.next_piece) {
                gs.is_game_over = true;
                break;
            }
            gs.counter = 0;
        }

        // handle input
        handle_input(
            &mut gs.display,
            key,
            &mut gs.active_piece,
            &mut gs.next_piece,
        );

        // hold piece
        if key == 'c' {
            hold(
                &mut gs.display,
                &mut gs.active_piece,
                &mut gs.hold_piece,
                &mut gs.next_piece,
            );
        }

        // full line
        full_line(&mut gs.display, &mut gs.gamescore);

        // ghost piece
        ghost_piece(&mut gs.display, &mut gs.active_piece);

        // update grav_tick
        grav_tick = 250 - gs.gamescore.level * 8;

        // check if gs.display was changed
        let is_updated = gs != prev_gs || gs.is_game_over;

        // render
        render(
            &gs.display,
            is_updated,
            &mut gs.gamescore,
            &gs.hold_piece,
            &gs.next_piece,
        );
        WRITER.lock().flush();
        gs.counter += 1;
    }
    return;
}
