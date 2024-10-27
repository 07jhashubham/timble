use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="text-white mt-5 flex justify-between">
            <div><img src="navIcon.svg" alt="" class="w-6 h-auto mt-2" /></div>
            <div class="flex flex-col items-center text-2xl"><h1 class="leading-6">FIND YOUR</h1> <h1 class="text-[#E44F90]">MATCH</h1></div>
            <div><img src="bell.svg" alt="" class="w-6 h-auto mt-2"/></div>
        </div>
    }
}
