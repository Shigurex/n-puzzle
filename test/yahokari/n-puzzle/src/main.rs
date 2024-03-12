mod algorithm;
mod args;
mod n_puzzle;

use yew::prelude::*;
use yew::{html, Html};

pub use algorithm::{Algorithm, Heuristic};
pub use n_puzzle::{Move, Pos, Puzzle, PuzzleSettings};

use yew::{function_component, Properties};

#[function_component]
fn App() -> Html {
    html! {
        <>
            <main>
                <PuzzleComponent />
            </main>
        </>
    }
}

//#[function_component]
//fn PuzzleComponent() -> Html {
//    let state: UseStateHandle<Vec<Vec<usize>>> = use_state(|| vec![
//            vec![1, 2, 3],
//            vec![4, 5, 6],
//            vec![7, 8, 0],
//        ]
//    );
//    //let state: Vec<Vec<usize>> = vec![
//    //    vec![1, 2, 3],
//    //    vec![4, 5, 6],
//    //    vec![7, 8, 0],
//    //];
//    let size = state.len();

//    html! {
//        <>
//            <div class="matrix">
//                { for (0..size).map(|col| html! {
//                    <div class="column">
//                        { for (0..size).map(|row| html! {
//                            if state[col][row] == 0 {
//                                <button class="cell empty">
//                                    { state[col][row] }
//                                </button>
//                            } else {
//                                <button class="cell">
//                                    { state[col][row] }
//                                </button>
//                            }
//                        })}
//                    </div>
//                })}
//            </div>
//        </>
//    }
//}

#[function_component]
fn PuzzleComponent() -> Html {
    let state: UseStateHandle<Vec<Vec<usize>>> = use_state(|| vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 0],
    ]);
    //let state: UseStateHandle<Vec<Vec<usize>>> = use_state(|| Puzzle::new(PuzzleSettings::Size(3)).unwrap().get_state().clone());

    let size = state.len();

    let swap_empty = {
        let state = state.clone();
        Callback::from(move |(row, col): (usize, usize)| {
            let mut new_state = (*state).clone();
            // 空のマスの位置を見つける
            let (empty_row, empty_col) = new_state.iter().enumerate().find_map(|(i, row)| {
                row.iter().enumerate().find_map(|(j, &cell)| {
                    if cell == 0 { Some((i, j)) } else { None }
                })
            }).unwrap();

            // クリックされたマスが空のマスの隣接するマスかをチェック
            let is_adjacent = (empty_row as i32 - row as i32).abs() + (empty_col as i32 - col as i32).abs() == 1;
            
            // 隣接していれば入れ替える
            if is_adjacent {
                let tmp = new_state[empty_row][empty_col];
                new_state[empty_row][empty_col] = new_state[row][col];
                new_state[row][col] = tmp;
                //new_state.swap(empty_row, row);
                //new_state[empty_row].swap(empty_col, col);
                state.set(new_state);
            }
        })
    };

    html! {
        <>
            <div class="matrix">
                { for (0..size).map(|row| html! {
                    <div class="row">
                        { for (0..size).map(|col| {
                            let cell_value = state[row][col];
                            let on_click = swap_empty.reform(move |_| (row, col));
                            html! {
                                <button onclick={on_click} class={classes!("cell", if cell_value == 0 { "empty" } else { "" })}>
                                    { cell_value }
                                </button>
                            }
                        })}
                    </div>
                })}
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html
}

//#[function_component]
//fn HelloWorld(props: &Props) -> Html {
//    html! {
//        <div class="very-stylized-container">
//            { props.children.clone() } // you can forward children like this
//        </div>
//    }
//}

//#[function_component]
//fn SolvePuzzle(state: Vec<Vec<usize>>) -> Html {
//    let a = 1;
//    html! {
//        <>
//            <div>
//                <button>{a}</button>
//                <button>{2}</button>
//                <button>{3}</button>
//            </div>
//        </>
//    }
//}

fn main() {
    yew::Renderer::<App>::new().render();
}
