#![no_std]

use gstd::{exec, msg, prelude::*};
use pebbles_game_io::*;

static mut GAME_STATE: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    let init_data: PebblesInit = msg::load().expect("Failed to load init data");

    // Validate input data
    assert!(
        init_data.pebbles_count > 0,
        "Pebbles count must be greater than 0"
    );
    assert!(
        init_data.max_pebbles_per_turn > 0,
        "Max pebbles per turn must be greater than 0"
    );

    // Choose the first player randomly
    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    // Initialize game state
    let mut game_state = GameState {
        pebbles_count: init_data.pebbles_count,
        max_pebbles_per_turn: init_data.max_pebbles_per_turn,
        pebbles_remaining: init_data.pebbles_count,
        difficulty: init_data.difficulty.clone(),
        first_player: first_player.clone(),
        winner: None,
    };

    // If the first player is Program, make the first move
    if first_player == Player::Program {
        let pebbles_to_remove = match init_data.difficulty {
            DifficultyLevel::Easy => (get_random_u32() % init_data.max_pebbles_per_turn + 1) as u32,
            DifficultyLevel::Hard => {
                find_best_move(init_data.pebbles_count, init_data.max_pebbles_per_turn)
            }
        };
        game_state.pebbles_remaining -= pebbles_to_remove;
    }

    unsafe {
        GAME_STATE = Some(game_state);
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Failed to load action");

    unsafe {
        if let Some(ref mut game_state) = GAME_STATE {
            match action {
                PebblesAction::Turn(pebbles) => {
                    assert!(
                        pebbles > 0 && pebbles <= game_state.max_pebbles_per_turn,
                        "Invalid number of pebbles"
                    );
                    game_state.pebbles_remaining -= pebbles;
                    if game_state.pebbles_remaining == 0 {
                        game_state.winner = Some(Player::User);
                        msg::reply(PebblesEvent::Won(Player::User), 0)
                            .expect("Failed to send reply");
                        return;
                    }
                    // Program's turn
                    let pebbles_to_remove = match game_state.difficulty {
                        DifficultyLevel::Easy => {
                            get_random_u32() % game_state.max_pebbles_per_turn + 1
                        }
                        DifficultyLevel::Hard => find_best_move(
                            game_state.pebbles_remaining,
                            game_state.max_pebbles_per_turn,
                        ),
                    };
                    game_state.pebbles_remaining -= pebbles_to_remove;
                    if game_state.pebbles_remaining == 0 {
                        game_state.winner = Some(Player::Program);
                        msg::reply(PebblesEvent::Won(Player::Program), 0)
                            .expect("Failed to send reply");
                    } else {
                        msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0)
                            .expect("Failed to send reply");
                    }
                }
                PebblesAction::GiveUp => {
                    game_state.winner = Some(Player::Program);
                    msg::reply(PebblesEvent::Won(Player::Program), 0)
                        .expect("Failed to send reply");
                }
                PebblesAction::Restart {
                    difficulty,
                    pebbles_count,
                    max_pebbles_per_turn,
                } => {
                    game_state.pebbles_count = pebbles_count;
                    game_state.max_pebbles_per_turn = max_pebbles_per_turn;
                    game_state.pebbles_remaining = pebbles_count;
                    game_state.difficulty = difficulty;
                    game_state.winner = None;
                    game_state.first_player = if get_random_u32() % 2 == 0 {
                        Player::User
                    } else {
                        Player::Program
                    };
                }
            }
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    unsafe {
        if let Some(ref game_state) = GAME_STATE {
            msg::reply(game_state, 0).expect("Failed to send state");
        }
    }
}

fn find_best_move(pebbles_remaining: u32, max_pebbles_per_turn: u32) -> u32 {
    let target = (pebbles_remaining - 1) % (max_pebbles_per_turn + 1);
    if target == 0 {
        // If no strategic move is possible, remove a random valid number of pebbles
        get_random_u32() % max_pebbles_per_turn + 1
    } else {
        target
    }
}

#[cfg(not(test))]
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[cfg(test)]
fn get_random_u32() -> u32 {
    42
}
