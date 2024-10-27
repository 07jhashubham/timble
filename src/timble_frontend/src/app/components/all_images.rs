use leptos::*;

struct ImageData {
    id: u8,
    url: &'static str,
    alt: &'static str,
}

#[component]
pub fn All_images() -> impl IntoView {
    // Initialize image data
    let images = vec![
        ImageData { id: 1, url: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRKavXzb5xdAZuzfu7w2yKup2PMF5bZIw65eQ&s", alt: "this is from ipl" },
        ImageData { id: 2, url: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTgOD0abk6GHTIRcOLItB1UnxDP8NtyACkMCA&s", alt: "this is from midday" },
        ImageData { id: 3, url: "https://24ai.tech/en/wp-content/uploads/sites/3/2023/10/01_product_1_sdelat-izobrazhenie-1-1-3-scaled.jpg", alt: "this is from last Day" },
        ImageData { id: 4, url: "https://static-cse.canva.com/blob/1625993/ComposeStunningImages6.jpg", alt: "this is from ipl" },
        ImageData { id: 5, url: "https://images.ctfassets.net/h6goo9gw1hh6/5wl7KPvpM44dPJ3kwKfwTe/0eb029cd00424d1b1934d780f57bbc34/Aspect-Ration-Image-1to1.jpg?w=1600&h=1600&fl=progressive&q=70&fm=jpg", alt: "this is from midday" },
        ImageData { id: 6, url: "https://i.pinimg.com/736x/de/b4/df/deb4df530bfee88fe07b2b9d49e7abf2.jpg", alt: "this is from last Day" },
    ];

    view! {
       <div class="mt-7 flex overflow-y-auto mb-5 scroll-hidden">
            {
                images.into_iter().map(move |x| {
                    view! {
                            <img src={x.url} alt={x.alt} class="w-24 h-12 mr-8 bg-[url('/smallframe.svg')] object-cover bg-no-repeat bg-center p-0.5 rounded-lg" />
                       
                    }
                }).collect_view()
            }
        </div>
    }
}
