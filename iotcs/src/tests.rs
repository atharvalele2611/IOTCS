use std::fs;

use super::{Direction, Farm, FarmParseError, IotCS};
use puzzle::test::{MoveTree, MoveTreeVerifyError};

fn moves_to_string(ms: &[Direction]) -> String {
    let mut s = String::new();
    for m in ms {
        s.push_str(&format!("{}", m))
    }
    s
}

fn load(file_stem: &str) -> Result<Farm, String> {
    let src =
        fs::read_to_string(format!("./assets/{file_stem}.txt", file_stem = file_stem)).unwrap();
    match src.parse::<Farm>() {
        Ok(puzzle) => Ok(puzzle),
        Err(_) => Err(format!(
            "`Farm::from_str` of {file_stem} should not fail.",
            file_stem = file_stem
        )),
    }
}

fn parse_test(file_stem: &str, parse_test_res: Result<(), FarmParseError>) -> Result<(), String> {
    let src =
        fs::read_to_string(format!("./assets/{file_stem}.txt", file_stem = file_stem)).unwrap();
    match (src.parse::<Farm>(), parse_test_res) {
        (Err(_), Err(_)) => Ok(()),
        (Err(_), Ok(_)) => Err(format!(
            "`Farm::from_str` of {file_stem} should not fail.",
            file_stem = file_stem
        )),
        (Ok(_), Err(_)) => Err(format!(
            "`Farm::from_str` of {file_stem} should fail.",
            file_stem = file_stem
        )),
        (Ok(_), Ok(())) => Ok(()),
    }
}

fn check_test(
    file_stem: &str,
    pre_puzzle: &Farm,
    (mvs_soln, goal_soln_ufo): (Vec<Direction>, String),
) -> Result<(), String> {
    let puzzle = IotCS::new(pre_puzzle);
    match puzzle::check(puzzle, &mvs_soln) {
        None => {
            return Err(format!(
                "{file_stem} reference solution ({mvs_soln}) failed `puzzle::check`",
                file_stem = file_stem,
                mvs_soln = moves_to_string(&mvs_soln)
            ))
        }
        Some(goal_chk) => {
            let goal_chk_ufo = goal_chk.ufo_with_cattle_to_string();
            if goal_soln_ufo != goal_chk_ufo {
                return Err(format!(
                    "{file_stem} reference solution ({mvs_soln}) final UFO config ({goal_soln_ufo}) does not equal `puzzle::check` final UFO config ({goal_chk_ufo}).",
                    file_stem = file_stem,
                    mvs_soln = moves_to_string(&mvs_soln),
                    goal_soln_ufo = goal_soln_ufo,
                    goal_chk_ufo = goal_chk_ufo
                ));
            }
        }
    };
    Ok(())
}

fn solve_test(
    file_stem: &str,
    pre_puzzle: &Farm,
    soln: Option<(Vec<Direction>, String)>,
) -> Result<(), String> {
    match (puzzle::solve(IotCS::new(pre_puzzle)), soln) {
        (None, None) => Ok(()),
        (Some((mvs, _)), None) => Err(format!(
            "{file_stem} has solution ({mvs}), but reference has no solution; likely has an incorrect `IotCS::next` (invalid move) and/or an incorrect `IotCS::is_goal`.",
            file_stem = file_stem,
            mvs = moves_to_string(&mvs)
        )),
        (None, Some((mvs_soln, _))) => Err(format!(
            "{file_stem} has no solution, but reference has solution ({mvs_soln}).",
            file_stem = file_stem,
            mvs_soln = moves_to_string(&mvs_soln)
        )),
        (Some((mvs, goal)), Some((mvs_soln, goal_soln_ufo))) => {
            match puzzle::check(IotCS::new(pre_puzzle), &mvs) {
                None => {
                    return Err(format!(
                        "{file_stem} solution ({mvs}) failed `puzzle::check`.",
                        file_stem = file_stem,
                        mvs = moves_to_string(&mvs)
                    ))
                }
                Some(goal_chk) => {
                    if goal != goal_chk {
                        return Err(format!(
                            "{file_stem} solution ({mvs}) final gameboard does not equal `puzzle::check` final gameboard.",
                            file_stem = file_stem,
                            mvs = moves_to_string(&mvs)
                        ));
                    }
                }
            };
            let goal_ufo = goal.ufo_with_cattle_to_string();
            match mvs.len().cmp(&mvs_soln.len()) {
                std::cmp::Ordering::Greater => Err(format!(
                    "{file_stem} solution ({mvs}) is longer than reference solution ({mvs_soln}).",
                    file_stem = file_stem,
                    mvs = moves_to_string(&mvs),
                    mvs_soln = moves_to_string(&mvs_soln)
                )),
                std::cmp::Ordering::Equal => {
                    if goal_ufo == goal_soln_ufo {
                        Ok(())
                    } else {
                        Err(format!(
                            "{file_stem} solution ({mvs}) final UFO config ({goal_ufo}) does not equal reference solution ({mvs_soln}) final UFO config ({goal_soln_ufo}); likely has an incorrect `IotCS::next` (invalid move) and/or an incorrect `IotCS::is_goal`.",
                            file_stem = file_stem,
                            mvs = moves_to_string(&mvs),
                            goal_ufo = goal_ufo,
                            mvs_soln = moves_to_string(&mvs_soln),
                            goal_soln_ufo = goal_soln_ufo,
                        ))
                    }
                }
                std::cmp::Ordering::Less => Err(format!(
                    "{file_stem} solution ({mvs}) is shorter than reference solution ({mvs_soln}); likely has an incorrect `IotCS::next` (invalid move) and/or an incorrect `IotCS::is_goal`.",
                    file_stem = file_stem,
                    mvs = moves_to_string(&mvs),
                    mvs_soln = moves_to_string(&mvs_soln)
                )),
            }
        }
    }
}

fn moves_test(
    file_stem: &str,
    puzzle: &Farm,
    move_tree: &MoveTree<Direction, String>,
) -> Result<(), String> {
    match move_tree.verify::<_, _, ()>(&IotCS::new(puzzle), |q, q_res| {
        if &q.ufo_with_cattle_to_string() == q_res {
            Ok(())
        } else {
            Err(())
        }
    }) {
        Ok(()) => Ok(()),
        Err((ms, err)) => {
            let state = if ms.is_empty() {
                file_stem.to_string()
            } else {
                format!(
                    "{file_stem} with moves {mvs}",
                    file_stem = file_stem,
                    mvs = moves_to_string(&ms)
                )
            };
            match err {
                MoveTreeVerifyError::MissingMove(m) => Err(format!(
                    "{state}, `IotCS::next` is missing move {m}",
                    state = state,
                    m = moves_to_string(&[m])
                )),
                MoveTreeVerifyError::ExtraMove(m) => Err(format!(
                    "{state}, `IotCS::next` has invalid move {m}",
                    state = state,
                    m = moves_to_string(&[m])
                )),
                MoveTreeVerifyError::DuplicateMove(m) => Err(format!(
                    "{state}, `IotCS::next` has duplicate move {m}",
                    state = state,
                    m = moves_to_string(&[m])
                )),
                MoveTreeVerifyError::ChkState(m, _) => Err(format!(
                    "{state}, `IotCS::next` includes move {m} leading to state with an incorrect UFO config",
                    state = state,
                    m = moves_to_string(&[m])
                )),
            }
        }
    }
}

mod badinput00 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput00", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput01 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput01", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput02 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput02", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput03 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput03", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput04 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput04", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput05 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput05", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput06 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput06", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput07 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput07", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput08 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput08", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput09 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput09", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput10 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput10", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod challenge {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("challenge", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("challenge")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/challenge.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("challenge", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("challenge", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("challenge", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 21731
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/challenge.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod easy00 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("easy00", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("easy00")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy00.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("easy00", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("easy00", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("easy00", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 7849
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy00.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod easy01 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("easy01", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("easy01")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy01.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("easy01", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("easy01", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("easy01", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 36272
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy01.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod easy10 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("easy10", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("easy10")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy10.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("easy10", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("easy10", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("easy10", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 66513
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy10.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod hard21 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("hard21", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("hard21")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/hard21.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("hard21", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("hard21", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("hard21", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 11615
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/hard21.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod hard30 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("hard30", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("hard30")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/hard30.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("hard30", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("hard30", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("hard30", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 61079
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/hard30.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod impossible {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("impossible", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("impossible")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/impossible.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("impossible", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("impossible", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 36104
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/impossible.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod medium11 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("medium11", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("medium11")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/medium11.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("medium11", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("medium11", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("medium11", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 148753
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/medium11.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod medium20 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("medium20", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("medium20")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/medium20.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("medium20", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("medium20", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("medium20", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 20875
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/medium20.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod superhard31 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("superhard31", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("superhard31")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/superhard31.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("superhard31", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("superhard31", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("superhard31", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 103913
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/superhard31.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod superhard40 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("superhard40", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("superhard40")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/superhard40.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("superhard40", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("superhard40", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("superhard40", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 90453
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/superhard40.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod trivial {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("trivial", Ok(()))
    }
    fn puzzle() -> Result<Farm, String> {
        load("trivial")
    }
    fn soln() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn check() -> Result<(), String> {
        check_test("trivial", &puzzle()?, soln().unwrap())
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("trivial", &puzzle()?, soln())
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("trivial", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 18338
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
