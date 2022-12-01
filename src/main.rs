use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <h1>{ "Rewriting this page in Rust/WASM with the Yew Framework." }</h1>
        <h3>{"About Me: Daniel J. Lee"}</h3>
        <p>{"As an integration engineer I plan, develop, integrate, test, deploy, and maintain software with various types of hardware and network systems. I specialize in Red Hat Enterprise Linux (RHEL) and wear multiple hats in the OS, application, and networking levels and apply DevOps practices to integrate them together."}</p>
        <p>{"I graduated from George Mason University with a Bachelor of Science Degree in Information Technology in December of 2020."}</p>
        </>
       
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}

