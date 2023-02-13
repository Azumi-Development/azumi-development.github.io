use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class={classes!("sticky", "bg-white/80", "backdrop-blur-md", "py-2")}>
            <div>
                <div class={classes!("container", "mx-40", "text-2xl", "w-20", "h-14", "mobile:hidden")}>
                    <img src="static/topicon.gif" class={classes!("rounded-full")} />
                </div>
            </div>
        </header>
    }
}

pub fn home() -> Html {
    html! {
        <div class={classes!("relative", "bg-white", "font-brand", "antialiased", "text-gray-850", "mobile:px-10")}>
            <Header />
            <h1 class={classes!("text-center", "pt-40", "text-6xl", "font-semibold")}>
                {"Imagine a safe space..."}
            </h1>
            <div class={classes!("text-center", "pt-10", "text-2xl", "max-w-5xl", "m-auto")}>
                {"...where you can have really fun commands, where you can have powerful moderation commands, and more! Well, that's Azumi!"}
            </div>
        </div>
    }
}
