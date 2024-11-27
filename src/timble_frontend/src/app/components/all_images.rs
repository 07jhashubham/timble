use leptos::*;
use crate::app::components::choose::ImageData;

#[component]
pub fn All_images(images:Vec<ImageData>) -> impl IntoView {
    view! {
       <div class="mt-7 flex overflow-y-auto scroll-hidden tppsr">
            {
                images.into_iter().map(move |x| {
                    view! {
                            <img src={x.url} alt={x.alt} class="w-12 h-12 flex-shrink-0 mr-8 bg-[url('/smallframe.svg')] object-cover bg-no-repeat bg-center p-0.5 rounded-lg" />
                       
                    }
                }).collect_view()
            }
        </div>
    }
}
