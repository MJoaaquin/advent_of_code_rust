use std::fs;

fn main() {
    #[derive(Debug)]
    enum Opponent {
        A,
        B,
        C,
    }

    #[derive(Debug)]
    enum My {
        Y,
        X,
        Z,
    }

    enum HandShape {
        Rock,
        Paper,
        Scissors,
    }

    impl HandShape {
        fn score(&self) -> u64 {
            match self {
                Self::Rock => 1,
                Self::Paper => 2,
                Self::Scissors => 3,
            }
        }

        fn outcome_of_the_round(opponent: HandShape, my: HandShape) -> u64 {
            match (opponent, my) {
                (Self::Rock, Self::Rock) => 3,
                (Self::Paper, Self::Paper) => 3,
                (Self::Scissors, Self::Scissors) => 3,
                (Self::Rock, Self::Paper) => 6,
                (Self::Rock, Self::Scissors) => 0,
                (Self::Paper, Self::Rock) => 0,
                (Self::Paper, Self::Scissors) => 6,
                (Self::Scissors, Self::Rock) => 6,
                (Self::Scissors, Self::Paper) => 0,
            }
        }

        fn handshape_by_opponent_handshape(opponent: HandShape, my: &My) -> HandShape {
            match (opponent, my) {
                (HandShape::Rock, My::X) => HandShape::Scissors,
                (HandShape::Paper, My::X) => HandShape::Rock,
                (HandShape::Scissors, My::X) => HandShape::Paper,
                (HandShape::Rock, My::Y) => HandShape::Rock,
                (HandShape::Paper, My::Y) => HandShape::Paper,
                (HandShape::Scissors, My::Y) => HandShape::Scissors,
                (HandShape::Rock, My::Z) => HandShape::Paper,
                (HandShape::Paper, My::Z) => HandShape::Scissors,
                (HandShape::Scissors, My::Z) => HandShape::Rock,
            }
        }
    }

    impl Opponent {
        fn handshape(&self) -> HandShape {
            match self {
                Self::A => HandShape::Rock,
                Self::B => HandShape::Paper,
                Self::C => HandShape::Scissors,
            }
        }

        fn to_variant(string: &str) -> Opponent {
            match string {
                "A" => Opponent::A,
                "B" => Opponent::B,
                "C" => Opponent::C,
                _ => panic!("option does not exist"),
            }
        }
    }

    impl My {
        fn handshape_for_part_1(&self) -> HandShape {
            match self {
                Self::X => HandShape::Rock,
                Self::Y => HandShape::Paper,
                Self::Z => HandShape::Scissors,
            }
        }

        fn handshape_for_part_2(&self, opponent_handshape: HandShape) -> HandShape {
            HandShape::handshape_by_opponent_handshape(opponent_handshape, self)
        }

        fn to_variant(string: &str) -> My {
            match string {
                "X" => My::X,
                "Y" => My::Y,
                "Z" => My::Z,
                _ => panic!("option does not exist"),
            }
        }
    }

    fn get_rounds_results(source_path: &str) -> Vec<(Opponent, My)> {
        let file_content = fs::read_to_string(source_path).unwrap();
        let vec_of_rounds: Vec<Vec<&str>> = file_content
            .split("\n")
            .map(|str| str.split(" ").collect())
            .collect();

        vec_of_rounds
            .iter()
            .map(|values| {
                (
                    Opponent::to_variant(values.first().unwrap()),
                    My::to_variant(values.last().unwrap()),
                )
            })
            .collect::<Vec<(Opponent, My)>>()
    }

    fn calculate_round_score_part_1(round: &(Opponent, My)) -> u64 {
        let (opponent, my) = round;
        HandShape::outcome_of_the_round(opponent.handshape(), my.handshape_for_part_1())
            + my.handshape_for_part_1().score()
    }

    fn calculate_round_score_part_2(round: &(Opponent, My)) -> u64 {
        let (opponent, my) = round;
        let my_handshape = my.handshape_for_part_2(opponent.handshape());
        let my_handshape_score = &my_handshape.score();
        HandShape::outcome_of_the_round(opponent.handshape(), my_handshape) + my_handshape_score
    }

    let rounds = get_rounds_results("./src/source.txt");
    let result_part_1: u64 = rounds
        .iter()
        .map(|round| calculate_round_score_part_1(round))
        .sum();

    let result_part_2: u64 = rounds
        .iter()
        .map(|round| calculate_round_score_part_2(round))
        .sum();

    println!("your score: {}", result_part_1);
    println!("your score: {}", result_part_2);
}
