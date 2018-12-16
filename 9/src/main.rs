use std::cmp;
use std::collections::HashMap;

fn add_score(player_scores: &mut HashMap<usize, usize>, player: usize, score: usize) {
    if player_scores.contains_key(&player) {
        *player_scores.get_mut(&player).unwrap() += score;
    } else {
        player_scores.insert(player, score);
    }
}

fn run_game(turns: usize, number_of_players: usize) {
    let mut player_scores = HashMap::new();
    let mut game_state = vec![0];

    let mut insert_index = 0;

    let mut highest_score = 0;
    for turn in 1..=turns {
        let len = game_state.len();
        if turn % 23 == 0 {
            if insert_index < 7 {
                insert_index += len;
            }
            insert_index = (insert_index - 7) % len;
            let marble = game_state.remove(insert_index);
            let player = turn % number_of_players;
            add_score(&mut player_scores, player, marble + turn);
            highest_score = cmp::max(highest_score, *player_scores.get(&player).unwrap());
        } else {
            let index = (insert_index + 2) % len;
            game_state.insert(index, turn);
            insert_index = index;
        }
    }

    for (player, score) in &player_scores {
        if *score == highest_score {
            println!("winning elf {} with {}", player, score);
            break;
        }
    }
}

fn main() {
    run_game(70953, 405);
    run_game(70953 * 100, 405);
}
