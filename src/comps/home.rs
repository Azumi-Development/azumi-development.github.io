use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class={classes!("sticky", "bg-white/80", "backdrop-blur-md", "py-2")}>
            <div>
                <div class={classes!("container", "desktop:mx-60", "tablet:mx-36", "text-2xl", "w-20", "h-14", "mobile:hidden")}>
                    <img src="static/topicon.gif" class={classes!("rounded-full")} />
                </div>
            </div>
        </header>
    }
}

pub fn home() -> Html {
    html! {
        <div class={classes!("relative", "font-brand", "antialiased", "mobile:px-10")}>
            <div class={classes!("text-gray-850")}>
                <Header />
                <h1 class={classes!("text-center", "pt-40", "text-6xl", "font-semibold")}>
                    {"Imagine a safe space..."}
                </h1>
                <div class={classes!("text-center", "pt-10", "text-2xl", "max-w-5xl", "m-auto")}>
                    {"...where you can have really fun commands, where you can have powerful moderation commands, and more! Well, that's Azumi!"}
                </div>
            </div>
            <div class={classes!("flex", "justify-center", "items-center", "pt-10", "text-xl", "gap-x-8")}>
                <a href="https://discord.com/api/oauth2/authorize?client_id=1005312021836869795&permissions=1099511627775&scope=bot">
                    <button class={classes!("transition", "duration-300", "rounded-full", "border", "border-primary", "border-black", "px-6", "py-2", "text-[#5865F2]", "hover:border-[#5865F2]", "hover:text-white", "hover:bg-[#5865F2]")}>
                        {"Invite Azumi"}
                    </button>
                </a>
                <a href="https://discord.gg/MSc4TZYuxB">
                    <button class={classes!("transition", "duration-300", "rounded-full", "bg-[#5865F2]", "px-6", "py-2", "text-white", "hover:bg-[#454FBF]")}>
                        {"Join the Discord"}
                    </button>
                </a>
            </div>
        </div>
    }
}
