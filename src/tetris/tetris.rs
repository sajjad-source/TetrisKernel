// Tetris

mod gamescore;
mod gamestate;
mod tetlib;
mod tetrominoe;

use gamestate::GameState;
use tetlib::*;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

fn run() {
    const MAX_LEVEL: usize = 20;
    const GRAV_TICK: usize = 40;
    const LEVEL_MULT: f64 = 0.85;

    // main loop
    loop {
        let prev_display = gs.display.clone();

        // handle input
        let key = get_input();

        // quit
        if key == 'q' {
            break;
        }

        // gravity
        if gs.counter >= (GRAV_TICK as f64 * LEVEL_MULT.powf(gs.gamescore.level as f64)) as usize {
            if gravity(&mut gs.display, &mut gs.active_piece, &mut gs.next_piece) {
                gs.is_game_over = true;
                break;
            }
            gs.counter = if gs.gamescore.level < MAX_LEVEL {
                0
            } else {
                100
            };
        }

        // handle input
        handle_input(
            &mut gs.display,
            key,
            &mut gs.active_piece,
            &mut gs.next_piece,
        );

        // hold piece
        if key == 'c' && !args.hold {
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
        if !args.ghost {
            ghost_piece(&mut gs.display, &mut gs.active_piece);
        }

        // check if gs.display was changed
        let is_updated = gs.display != prev_display || gs.is_game_over;

        // render
        render(
            &gs.display,
            is_updated,
            &mut gs.gamescore,
            &gs.hold_piece,
            &gs.next_piece,
        );
        sleep(Duration::from_millis(args.gravity));
        gs.counter += 1;
    }

    print!("{}", "\n".repeat(HEIGHT / 2 + 4));
}
