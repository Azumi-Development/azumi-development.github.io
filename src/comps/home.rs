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
        <div>
        <div class={classes!("relative", "font-brand", "antialiased", "mobile:px-10")}>
        <div class={classes!("text-gray-850")}>
        <Header />
            <h1 class={classes!("text-center", "pt-40", "text-6xl", "font-semibold")}>
                    {"Imagine a safe space..."}
            </h1>
        </div>
        </div>
        <div class={classes!("flex", "justify-center", "items-center", "pt-10", "text-xl", "gap-x-8")}>
        <a href="https://discord.com/api/oauth2/authorize?client_id=1005312021836869795&permissions=1099511627775&scope=bot">
            <button class={classes!("transition", "duration-300", "rounded-full", "border", "border-primary", "border-black", "px-6", "py-2", "text-[#5865F2]", "hover:border-[#5865F2]", "hover:text-white", "hover:bg-[#5865F2]")}>
                {"Add to Server"}
            </button>
        </a>
        <a href="https://discord.gg/xuqmUTvDqq">
            <button class={classes!("transition", "duration-300", "rounded-full", "bg-[#5865F2]", "px-6", "py-2", "text-white", "hover:bg-[#454FBF]")}>
                {"Join the Discord"}
            </button>
        </a>
        </div>
    </div>
        <div>
        <h1 class={classes!("text-center","strong")}>
        {"Before doing anything, read these:"}
        </h1>
        </div>
        <div class={classes!("text-center","strong")}>
        <a target="_blank" href="https://raw.githubusercontent.com/Azumi-Development/Azumi/main/terms-of-service">{"Terms of Service"}</a><br/>
        <a target="_blank" href="https://raw.githubusercontent.com/Azumi-Development/Azumi/main/privacy-policy">{"Privacy Policy"}</a><br/>
        </div>
        <div>
        <h3 class={classes!("text-center","strong")}>
        {"Thank you for reading!"}
        </h3>
        </div>
        <div class={classes!("text-center","font-semibold")}>
        <h2>
        {"What is Azumi?"}
        </h2>
        </div>
        <div class={classes!("text-center")}
        {"Azumi is a Discord bot shaped and made by the community. It is written in "}
        <a target="_blank" href="https://python.org">
        {"Python"}
        </a>
        {"v3, with the help of "}
        <a target="_blank" href="https://pycord.dev/">
        {"Pycord"}
        </a>
        {"v2."}
        <br/>
        {"Azumi is focused on being a widely available and easy-to-use and customize, utility bot. It has several features to help with your day-to-day Discord use, such as:"}
        <br/>
        <br/>
        <br/>
        {"Timestamp Generation"}
        <br/>
        {"Moderation Commands"}
        <br/>
        {"Weather Information"}
        <br/>
        {"Member Data"}
        <br/>
        {"GitHub Information"}
        <br/>
        {"Message Grabbing"}
        <br/>
        {"Math Calculation"}
        <br/>
        {"Dice Rolling"}
        <br/>
        {"Pronoun Role Management (Beta)"}
        <br/>
        {"Server Logs (Coming Soon!)"}
        <br/>
        <br/>
        <br/>
        {"There's also a lot planned for the future..."}
        </div>
        </div>
        <br/>
        <br/>
        {"© Azumi Development 2022-2023"}
        </div>
        }
    }
