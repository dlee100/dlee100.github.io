use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <head>
            <meta charset="UTF-8"/>
            <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            //<!-- Begin Jekyll SEO tag v2.7.1 -->
            <title>{"Daniel Lee"}</title>
            <meta name="generator" content="Jekyll v3.9.0"/>
            <meta property="og:title" content="Welcome to GitHub Pages"/>
            <meta property="og:locale" content="en_US"/>
            <meta name="description" content="Static site"/>
            <meta property="og:description" content="Static site"/>
            <link rel="canonical" href="https://dlee100.github.io/"/>
            <meta property="og:url" content="https://dlee100.github.io/"/>
            <meta property="og:site_name" content="dlee100.github.io"/>
            <meta name="twitter:card" content="summary"/>
            <meta property="twitter:title" content="Welcome to GitHub Pages"/>
            <script type="application/ld+json" src="app.json"/>
            //</script>
            //<!-- End Jekyll SEO tag -->
            <link rel="stylesheet" href="/assets/css/style.css?v=3a92262abbc34cc97bfec8c43c283bff5cc4d4d3"/>
        </head>

        <body>
            <div class="wrapper">
                <header>
                    <h1><a href="https://dlee100.github.io/">{"Daniel Lee"}</a></h1>
                
                    <p class="view"><a href="src/pdf/daniel-lee-resume-202212.pdf">{"View my official resume"}</a></p>
    
                    <p>{"My github.io site. Work in progess!"}</p>
                
                    <p class="view"><a href="https://github.com/dlee100">{"View My GitHub Profile"}</a></p>
                </header>
               
                <section>
                    <h1>{ "Rewriting this page in Rust/WASM with the Yew Framework." }</h1>
                    <h3>{"About Me: Daniel J. Lee"}</h3>
                    <p>{"As an integration engineer I plan, develop, integrate, test, deploy, and maintain software with various types of hardware and network systems. I specialize in Red Hat Enterprise Linux (RHEL) and wear multiple hats in the OS, application, and networking levels and apply DevOps practices to integrate them together."}</p>
                    <p>{"I graduated from George Mason University with a Bachelor of Science Degree in Information Technology in December of 2020."}</p>
                </section>
                <footer>
                    <p>
                        <small>
                            {"Hosted on GitHub Pages — Theme by "} 
                        </small>
                        <small>
                            <a href="https://github.com/orderedlist">{"orderedlist"}</a>
                        </small>
                    </p>
                </footer>
            </div>
            <script src="/assets/js/scale.fix.js"/>
            //</script>

        </body>
        </>
       
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}

