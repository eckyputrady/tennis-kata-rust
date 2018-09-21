#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Point {
    P0, P15, P30, P40, PAdv
}

#[derive(PartialEq)]
#[derive(Debug)]
enum GameResult {
    P1Win, P2Win, OnGoing
}

#[derive(PartialEq)]
#[derive(Debug)]
enum GameUpdate {
    P1Score, P2Score
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Game {
    player_1: Point,
    player_2: Point,
    result: GameResult
}

fn update_game(game: &mut Game, update: &GameUpdate) {
    if game.result != GameResult::OnGoing {
        return;
    }

    match update {
        GameUpdate::P1Score => {
            adv_p1_score(game);
        },
        GameUpdate::P2Score => {
            swap_score(game);
            adv_p1_score(game);
            swap_score(game);
        }
    }
}

fn swap_score(game: &mut Game) {
    let temp = game.player_1;
    game.player_1 = game.player_2;
    game.player_2 = temp;
    game.result = match game.result {
        GameResult::P1Win => GameResult::P2Win,
        GameResult::P2Win => GameResult::P1Win,
        GameResult::OnGoing => GameResult::OnGoing
    }
}

fn adv_p1_score(game: &mut Game) {
    match (&game.player_1, &game.player_2) {
        (Point::PAdv, _) => {
            game.result = GameResult::P1Win;
        },
        (Point::P40, Point::PAdv) => {
            game.player_2 = Point::P40;
        },
        (Point::P40, Point::P40) => {
            game.player_1 = Point::PAdv;
        },
        (Point::P40, _) => {
            game.result = GameResult::P1Win;
        },
        (Point::P30, _) => {
            game.player_1 = Point::P40;
        },
        (Point::P15, _) => {
            game.player_1 = Point::P30;
        },
        (Point::P0, _) => {
            game.player_1 = Point::P15;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tennis_test_p1() {
        generic_tennis_test_p1(Point::P0, Point::P0, Point::P15, Point::P0, false);
        generic_tennis_test_p1(Point::P15, Point::P0, Point::P30, Point::P0, false);
        generic_tennis_test_p1(Point::P30, Point::P0, Point::P40, Point::P0, false);
        generic_tennis_test_p1(Point::P40, Point::P0, Point::P40, Point::P0, true);
        generic_tennis_test_p1(Point::P15, Point::P15, Point::P30, Point::P15, false);
        generic_tennis_test_p1(Point::P30, Point::P15, Point::P40, Point::P15, false);
        generic_tennis_test_p1(Point::P40, Point::P15, Point::P40, Point::P15, true);
        generic_tennis_test_p1(Point::P15, Point::P30, Point::P30, Point::P30, false);
        generic_tennis_test_p1(Point::P30, Point::P30, Point::P40, Point::P30, false);
        generic_tennis_test_p1(Point::P40, Point::P30, Point::P40, Point::P30, true);
        generic_tennis_test_p1(Point::P15, Point::P40, Point::P30, Point::P40, false);
        generic_tennis_test_p1(Point::P30, Point::P40, Point::P40, Point::P40, false);
        generic_tennis_test_p1(Point::P40, Point::P40, Point::PAdv, Point::P40, false);
        generic_tennis_test_p1(Point::P40, Point::P40, Point::PAdv, Point::P40, false);
        generic_tennis_test_p1(Point::P40, Point::PAdv, Point::P40, Point::P40, false);
    }

    #[test]
    fn tennis_test_p2() {
        generic_tennis_test_p2(Point::P0, Point::P0, Point::P15, Point::P0, false);
        generic_tennis_test_p2(Point::P15, Point::P0, Point::P30, Point::P0, false);
        generic_tennis_test_p2(Point::P30, Point::P0, Point::P40, Point::P0, false);
        generic_tennis_test_p2(Point::P40, Point::P0, Point::P40, Point::P0, true);
        generic_tennis_test_p2(Point::P15, Point::P15, Point::P30, Point::P15, false);
        generic_tennis_test_p2(Point::P30, Point::P15, Point::P40, Point::P15, false);
        generic_tennis_test_p2(Point::P40, Point::P15, Point::P40, Point::P15, true);
        generic_tennis_test_p2(Point::P15, Point::P30, Point::P30, Point::P30, false);
        generic_tennis_test_p2(Point::P30, Point::P30, Point::P40, Point::P30, false);
        generic_tennis_test_p2(Point::P40, Point::P30, Point::P40, Point::P30, true);
        generic_tennis_test_p2(Point::P15, Point::P40, Point::P30, Point::P40, false);
        generic_tennis_test_p2(Point::P30, Point::P40, Point::P40, Point::P40, false);
        generic_tennis_test_p2(Point::P40, Point::P40, Point::PAdv, Point::P40, false);
        generic_tennis_test_p2(Point::P40, Point::P40, Point::PAdv, Point::P40, false);
        generic_tennis_test_p2(Point::P40, Point::PAdv, Point::P40, Point::P40, false);
    }

    fn generic_tennis_test_p1(p1: Point, p2: Point, p1_result: Point, p2_result: Point, is_win: bool) {
        generic_tennis_test(p1, p2, GameUpdate::P1Score, p1_result, p2_result, if is_win { GameResult::P1Win } else { GameResult::OnGoing})
    }

    fn generic_tennis_test_p2(p1: Point, p2: Point, p1_result: Point, p2_result: Point, is_win: bool) {
        generic_tennis_test(p2, p1, GameUpdate::P2Score, p2_result, p1_result, if is_win { GameResult::P2Win } else { GameResult::OnGoing})
    }

    fn generic_tennis_test(p1: Point, p2: Point, update: GameUpdate, p1_result: Point, p2_result: Point, game_result: GameResult) {
        let mut actual_game = Game { player_1: p1, player_2: p2, result: GameResult::OnGoing };
        update_game(&mut actual_game, &update);
        let expected_game = Game { player_1: p1_result, player_2: p2_result, result: game_result };
        assert_eq!(&actual_game, &expected_game);
    }
}