fn main() {
    enum Opponent {
        A,
        B,
        C,
    }

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
        fn score(&self) -> u8 {
            match self {
                Self::Rock => 1,
                Self::Paper => 2,
                Self::Scissors => 3,
            }
        }

        fn outcome_of_the_round(opponent: HandShape, my: HandShape) -> u8 {
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
        fn handshape(&self) -> HandShape {
            match self {
                Self::X => HandShape::Rock,
                Self::Y => HandShape::Paper,
                Self::Z => HandShape::Scissors,
            }
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

    fn calculate() {
        // let my_choices = vec![My::Y, My::X, My::Z];
        // let opponent_choices = vec![Opponent::A, Opponent::B, Opponent::C];

        let round_1 = HandShape::outcome_of_the_round(Opponent::A.handshape(), My::X.handshape())
            + My::X.handshape().score();

        let round_2 = HandShape::outcome_of_the_round(Opponent::B.handshape(), My::Y.handshape())
            + My::Y.handshape().score();

        let round_3 = HandShape::outcome_of_the_round(Opponent::C.handshape(), My::Z.handshape())
            + My::Z.handshape().score();

        println!("{}", round_1 + round_2 + round_3)
    }

    calculate()
}
