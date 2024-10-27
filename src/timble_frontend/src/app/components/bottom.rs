
use leptos::*;

#[component]
pub fn Bottom() -> impl IntoView {
    view! {
        <div class=" flex justify-around items-center els">

        <div class="relative inline-block">
            <img src="ch1.svg" alt="rgh" class="w-full" />
            <img src="ch2.svg" alt="cross" class="absolute inset-0 m-auto" />
        </div>
        <div class="relative inline-block">
            <img src="ch4.svg" alt="rgh" class="w-full" />
            <img src="ch5.svg" alt="cross" class="absolute inset-0 m-auto" />
        </div>
        
        
        <div class="relative inline-block">
            <img src="ch1.svg" alt="rgh" class="w-full" />
            <img src="ch3.svg" alt="cross" class="absolute inset-0 m-auto" />
        </div>
        
        </div>
    }
}