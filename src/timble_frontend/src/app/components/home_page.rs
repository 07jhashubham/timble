use leptos::*;
use crate::app::components::navbar::Navbar;
use crate::app::components::all_images::All_images;
use crate::app::components::crisp::Crisp;
use crate::app::components::bottom::Bottom;
use crate::app::components::sec_nav::SecNav;
use crate::app::components::sec_allimg::SecAllimg;
use crate::app::components::sec_detail::SecDetail;
use crate::app::components::choose::ImageData;

#[component]
pub fn HomePage() -> impl IntoView {
    // Signal for toggling state
    let (component, set_component) = create_signal(false);

    // Images vector
    let images = vec![
        ImageData { id: 1, url: "main.png", alt: "this is from ipl" },
        ImageData { id: 2, url: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTgOD0abk6GHTIRcOLItB1UnxDP8NtyACkMCA&s", alt: "this is from midday" },
        ImageData { id: 3, url: "https://24ai.tech/en/wp-content/uploads/sites/3/2023/10/01_product_1_sdelat-izobrazhenie-1-1-3-scaled.jpg", alt: "this is from last Day" },
        ImageData { id: 4, url: "https://static-cse.canva.com/blob/1625993/ComposeStunningImages6.jpg", alt: "this is from ipl" },
        ImageData { id: 5, url: "https://images.ctfassets.net/h6goo9gw1hh6/5wl7KPvpM44dPJ3kwKfwTe/0eb029cd00424d1b1934d780f57bbc34/Aspect-Ration-Image-1to1.jpg?w=1600&h=1600&fl=progressive&q=70&fm=jpg", alt: "this is from midday" },
        ImageData { id: 6, url: "https://i.pinimg.com/736x/de/b4/df/deb4df530bfee88fe07b2b9d49e7abf2.jpg", alt: "this is from last Day" },
    ];

    view! {
        <div class="w-full bg-gray-200 flex items-center justify-center">
            <div class="bg-[#05102E] md:max-w-md md:rounded-2xl w-full h-screen overflow-y-auto scroll-hidden">
                {move || if component.get() {
                    // When the secondary content is displayed
                    view! {
                        <div class="mx-6">
                            <SecNav check=Box::new(move || set_component(false)) />
                            <SecAllimg images=images.clone() />
                            <SecDetail />
                        </div>
                    }
                } else {
                    // Default content
                    view! {
                        <div class="mx-6">
                            <Navbar />
                            <All_images images=images.clone() />
                            <Crisp check=Box::new(move || set_component(true)) /> // Pass set_component as a callback
                            <Bottom />
                        </div>
                    }
                }}
            </div>
        </div>
    }
}