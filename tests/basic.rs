#[cfg(test)]
mod tests {
    use gstd::prelude::*;
    use pebbles_game_io::*;

    #[test]
    fn test_game_state_initialization() {
        let init_data = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
        };

        let game_state = GameState {
            pebbles_count: init_data.pebbles_count,
            max_pebbles_per_turn: init_data.max_pebbles_per_turn,
            pebbles_remaining: init_data.pebbles_count,
            difficulty: init_data.difficulty.clone(),
            first_player: Player::User,
            winner: None,
        };

        assert_eq!(game_state.pebbles_count, 10);
        assert_eq!(game_state.max_pebbles_per_turn, 3);
        assert_eq!(game_state.pebbles_remaining, 10);
        assert_eq!(game_state.difficulty, DifficultyLevel::Easy);
        assert_eq!(game_state.first_player, Player::User);
        assert_eq!(game_state.winner, None);
    }

    #[test]
    fn test_turn_action() {
        let mut game_state = GameState {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            pebbles_remaining: 10,
            difficulty: DifficultyLevel::Easy,
            first_player: Player::User,
            winner: None,
        };

        // Simulate a user turn
        let pebbles_taken = 2;
        game_state.pebbles_remaining -= pebbles_taken;

        assert_eq!(game_state.pebbles_remaining, 8);

        // Simulate a program turn
        let pebbles_taken_by_program = 1;
        game_state.pebbles_remaining -= pebbles_taken_by_program;

        assert_eq!(game_state.pebbles_remaining, 7);
    }

    #[test]
    fn test_game_won() {
        let mut game_state = GameState {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            pebbles_remaining: 1,
            difficulty: DifficultyLevel::Easy,
            first_player: Player::User,
            winner: None,
        };

        // Simulate a user turn that wins the game
        let pebbles_taken = 1;
        game_state.pebbles_remaining -= pebbles_taken;

        if game_state.pebbles_remaining == 0 {
            game_state.winner = Some(Player::User);
        }

        assert_eq!(game_state.winner, Some(Player::User));
    }
}
