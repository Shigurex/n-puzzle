use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Puzzle Structure" }</h1>
            <div>
                <div>
                    <button>{ "1" }</button>
                    <button>{ "2" }</button>
                    <button>{ "3" }</button>
                </div>
                <div>
                    <button>{ "4" }</button>
                    <button>{ "5" }</button>
                    <button>{ "6" }</button>
                </div>
                <div>
                    <button>{ "7" }</button>
                    <button>{ "8" }</button>
                    <button>{ "0" }</button>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
