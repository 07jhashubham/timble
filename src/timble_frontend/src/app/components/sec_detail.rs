use leptos::*;

#[component]
pub fn SecDetail() -> impl IntoView {
    view! {
        <div class="text-white mt-4">
            <div class="flex flex-col">
               
                <p>
                    <span class="font-bold text-2xl mr-2">John Doe</span>
                    <span class="font-bold text-2xl">25</span>
                </p>
                <span class="font-bold text-lg mb-1">Fashion Designer</span>
                <div class="flex space-x-3">
                    <img src="loc.svg" alt="Location Icon" />
                    <span>32Km</span>
                </div>

               
                <div>
                    <h1 class="text-4xl mt-4 font-bold">Interests</h1>
                    <div class="grid grid-cols-3 gap-4 mt-4 text-center">
                        <p class="text-lg font-medium bg-[#495678] px-4 py-2 rounded-md">Designing</p>
                        <p class="text-lg font-medium bg-[#495678] px-4 py-2 rounded-md">Swimming</p>
                        <p class="text-lg font-medium bg-[#495678] px-4 py-2 rounded-md">Cycling</p>
                        <p class="text-lg font-medium bg-[#495678] px-4 py-2 rounded-md">Photography</p>
                        <p class="text-lg font-medium bg-[#495678] px-4 py-2 rounded-md">Guitar</p>
                    </div>
                </div>
                <div>
                    <h1 class="text-4xl mt-4 font-bold">About</h1>
                    <p class="mt-3 mb-5">Creative at heart, blending fashion design with a love for music and adventure. When Im not sketching my next design, Im either playing guitar, snapping photos, or swimming. </p>
                </div>
            </div>
        </div>
    }
}