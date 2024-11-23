use leptos::*;
use crate::app::components::navbar::Navbar;
use crate::app::components::all_images::All_images;
use crate::app::components::crisp::Crisp;
use crate::app::components::bottom::Bottom;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
       <div class="w-full bg-gray-200 flex items-center justify-center">
        <div class="bg-[#05102E] md:max-w-md md:rounded-2xl w-full h-screen overflow-y-auto">
            <div class="mx-6">  
                <Navbar />
                <All_images />
                <Crisp />
                <Bottom />
            </div>
        </div>
    </div>
    }
}

