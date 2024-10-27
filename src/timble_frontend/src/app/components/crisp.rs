use leptos::*;

#[component]
pub fn crisp() -> impl IntoView {
    view! {
       <div class="relative bg-gradient-to-t rounded-3xl from-[#E44F90] to-[#80B7F5] scale-[.68] check xl:-mt-[7.5rem] xl:scale-[.65] ">
    <img class="rounded-3xl p-2" src="main.png" alt="This is the image" />

    <div class="absolute bottom-4 left-4 right-4 flex justify-between items-center bg-opacity-75 rounded-xl p-3 text-white">
        <div class="flex flex-col">
            <p><span class="font-bold text-2xl mr-2">John Doe</span>
            <span class="font-bold text-2xl">25</span></p>
            <span class="font-bold text-lg mb-1">Fashon Designer</span>
            <div class="flex space-x-3"><img src="loc.svg" alt="loc" /> <span>32Km</span></div>
        </div>
        
        <button class="bg-gray-400 bg-opacity-70  rounded-lg px-6 py-4 hover:bg-gray-700">
            <img src="Arrow.svg" alt="upSide" />
        </button>
    </div>
</div>

    }
}