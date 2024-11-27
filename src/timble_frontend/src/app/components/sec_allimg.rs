use leptos::*;
use crate::app::components::choose::ImageData;
#[component]
pub fn sec_allimg(images: Vec<ImageData>) -> impl IntoView {
    
    view! {
        <div class=" flex overflow-y-scroll space-x-24 mt-4 scroll-hidden">
            {
                images.iter().map(|x| {
                    view! {
                        <img src={x.url} class=" rounded-[2.5rem] w-96 h-96 object-cover flex-shrink-0" alt={x.alt} />
                    }
                }).collect_view()
            }
        </div>
    }
}