mod algorithm;
mod args;
mod n_puzzle;

use anyhow::anyhow;
use yew::prelude::*;
use yew::{html, Html};

pub use algorithm::{Algorithm, Heuristic, Solver};
pub use n_puzzle::{Move, Pos, Puzzle, PuzzleSettings};

use yew::{function_component, Properties};

#[function_component]
fn App() -> Html {
    let size = use_state(|| 3);
    let puzzle = use_state(|| Puzzle::new(PuzzleSettings::Size(*size)).unwrap());

    let move_blank = {
        let puzzle = puzzle.clone();
        Callback::from(move |clicked_pos: Pos| {
            let blank_pos = puzzle.get_blank_pos();
            let (dx, dy) = (
                clicked_pos.x as isize - blank_pos.x as isize,
                clicked_pos.y as isize - blank_pos.y as isize,
            );

            let mut new_puzzle = (*puzzle).clone();
            let action = match (dx, dy) {
                (1, 0) => new_puzzle.move_blank(Move::Right),
                (-1, 0) => new_puzzle.move_blank(Move::Left),
                (0, 1) => new_puzzle.move_blank(Move::Down),
                (0, -1) => new_puzzle.move_blank(Move::Up),
                _ => Err(anyhow!("Invalid move")),
            };

            if action.is_ok() {
                puzzle.set(new_puzzle);
            }
        })
    };

    let solve_puzzle = {
        let puzzle = puzzle.clone();
        move |_| {
            let new_puzzle = (*puzzle).clone();
            let solver = Solver::new(Algorithm::AStar, Heuristic::Manhattan, new_puzzle, None);
            let output = match solver.solve(true) {
                Ok(output) => output,
                Err(e) => {
                    log::error!("Error solving puzzle: {:?}", e);
                    return; // ここで早期リターン
                }
            };
            let path = output.get_path();
            log::info!("Path: {:?}", path);
        }
    };

    html! {
        <>
            <main>
                <div class="matrix">
                    { for (0..(*puzzle).get_size()).map(|row| html! {
                        <div class="row">
                            { for (0..(*puzzle).get_size()).map(|col| {
                                let cell_value = puzzle.get(Pos::new(col, row)).unwrap();
                                let on_click = move_blank.reform(move |_| Pos::new(col, row));
                                html! {
                                    if cell_value == 0 {
                                        <button onclick={on_click} class={classes!("cell", "empty")}>
                                            { "" }
                                        </button>
                                    } else {
                                        <button onclick={on_click} class={classes!("cell")}>
                                            { cell_value }
                                        </button>
                                    }
                                }
                            })}
                        </div>
                    })}
                </div>
                if puzzle.is_final_state() {
                    <div class="win">
                        { "You win!" }
                    </div>
                }
                <div>
                    <button onclick={Callback::from(move |_| {
                        let new_puzzle = Puzzle::new(PuzzleSettings::Size(*size)).unwrap();
                        puzzle.set(new_puzzle);
                    })}>
                        { "Shuffle" }
                    </button>
                    <button onclick={solve_puzzle}>
                        { "Solve" }
                    </button>
                </div>
            </main>
        </>
    }
}

//#[derive(Properties, PartialEq)]
//pub struct PuzzleProps {
//    pub puzzle: Puzzle,
//}

//#[function_component]
//fn PuzzleComponent(props: &PuzzleProps) -> Html {
//    let puzzle = use_state(|| props.puzzle.clone());
//    let size = props.puzzle.get_size();

//    let move_blank = {
//        let puzzle = puzzle.clone();
//        Callback::from(move |clicked_pos: Pos| {
//            let blank_pos = puzzle.get_blank_pos();
//            let (dx, dy) = (clicked_pos.x as isize - blank_pos.x as isize, clicked_pos.y as isize - blank_pos.y as isize);

//            let mut new_puzzle = (*puzzle).clone();
//            let action = match (dx, dy) {
//                (1, 0) => new_puzzle.move_blank(Move::Right),
//                (-1, 0) => new_puzzle.move_blank(Move::Left),
//                (0, 1) => new_puzzle.move_blank(Move::Down),
//                (0, -1) => new_puzzle.move_blank(Move::Up),
//                _ => Err(anyhow!("Invalid move")),
//            };

//            if action.is_ok() {
//                puzzle.set(new_puzzle);
//            }
//        })
//    };

//    html! {
//        <>
//            <div class="matrix">
//                { for (0..size).map(|row| html! {
//                    <div class="row">
//                        { for (0..size).map(|col| {
//                            let cell_value = puzzle.get(Pos::new(col, row)).unwrap();
//                            let on_click = move_blank.reform(move |_| Pos::new(col, row));
//                            html! {
//                                if cell_value == 0 {
//                                    <button onclick={on_click} class={classes!("cell", "empty")}>
//                                        { "" }
//                                    </button>
//                                } else {
//                                    <button onclick={on_click} class={classes!("cell")}>
//                                        { cell_value }
//                                    </button>
//                                }
//                            }
//                        })}
//                    </div>
//                })}
//            </div>
//            if puzzle.is_final_state() {
//                <div class="win">
//                    { "You win!" }
//                </div>
//            }
//        </>
//    }
//}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
