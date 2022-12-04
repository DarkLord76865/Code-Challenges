fn main() {
    let input = include_str!("../input.txt");
    let mut score: u32 = 0;
    let mut game_values: Vec<&str>;
    for game in input.split("\n") {
        if game != "" {
            game_values = game.split(" ").collect();
            score += match game_values[1] {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!()
            };
            if (game_values[0] == "A" && game_values[1] == "Y") ||
                (game_values[0] == "B" && game_values[1] == "Z") ||
                (game_values[0] == "C") && game_values[1] == "X" {
                score += 6;
            } else if (game_values[0] == "A" && game_values[1] == "X") ||
                (game_values[0] == "B" && game_values[1] == "Y") ||
                (game_values[0] == "C") && game_values[1] == "Z" {
                score += 3;
            }
        }
    }
    println!("{}", score);
    score = 0;
    for game in input.split("\n") {
        if game != "" {
            game_values = game.split(" ").collect();
            score += match game_values[1] {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!()
            };
            score += match game_values[1] {
                "X" => {
                    match game_values[0] {
                        "A" => 3,
                        "B" => 1,
                        "C" => 2,
                        _ => panic!()
                    }
                },
                "Y" => {
                    match game_values[0] {
                        "A" => 1,
                        "B" => 2,
                        "C" => 3,
                        _ => panic!()
                    }
                },
                "Z" => {
                    match game_values[0] {
                        "A" => 2,
                        "B" => 3,
                        "C" => 1,
                        _ => panic!()
                    }
                },
                _ => panic!()
            };
        }
    }
    println!("{}", score);
}
