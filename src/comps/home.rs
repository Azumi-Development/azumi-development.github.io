use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class={classes!("sticky", "bg-white/80", "backdrop-blur-md", "py-2")}>
            <div>
                <div class={classes!("container", "desktop:ml-40", "text-2xl", "w-20", "h-14", "mobile:ml-10")}>
                    <img src="static/topicon.gif" class={classes!("rounded-full")} />
                </div>
            </div>
        </header>
    }
}

pub fn home() -> Html {
    html! {
        <div class={classes!("relative", "max-h", "min-h-screen", "bg-white", "font-brand", "antialiased", "text-gray-850", "mb:px-20")}>
            <Header />
            <h1 class={classes!("text-center", "pt-40", "text-6xl", "font-semibold")}>
                {"Imagine a safe space..."}
            </h1>
            <div class={classes!("text-center", "pt-10", "text-2xl", "max-w-5xl", "m-auto")}>
                {"...lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
            </div>
        </div>
    }
}
