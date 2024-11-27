use leptos::*;

#[component]
pub fn SecNav(check: Box<dyn Fn()>) -> impl IntoView {
    view! {
         <div class="text-white mt-5 flex justify-between">
            <div><img src="secnavtop.svg" alt="" class="w-10 h-auto mt-2" on:click=move |_| {check()} /></div>
            <div><img src="secnavtop2.svg" alt="" class="w-10 h-auto mt-2"/></div>
        </div>
    }
}