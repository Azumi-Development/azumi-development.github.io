use yew::prelude::*;

 
pub fn home() -> Html {
    html! {
        <div class={classes!("relative", "max-h", "min-h-screen", "bg-[#36393e]")}>
            <div class={classes!("text-center", "pt-40", "text-6xl", "font-sans", "text-white")}>
                {"Imagine a safe space..."}
            </div>
            <div class={classes!("text-center", "pt-10", "text-2xl", "font-sans", "text-white", "max-w-5xl", "m-auto")}>
                {"...lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
            </div>
        </div>
    }
}
