use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::podcasts::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: String,
    pub can_modify: bool,
}

/// Podcast actions component to edit or delete a podcast.
#[function_component(PodcastActions)]
pub fn podcast_actions(props: &Props) -> Html {
    let history = use_history().unwrap();
    let podcast_delete = {
        let slug = props.slug.clone();
        use_async(async move { del(slug).await })
    };
    let onclick = {
        let podcast_delete = podcast_delete.clone();
        Callback::from(move |_| {
            podcast_delete.run();
        })
    };

    use_effect_with_deps(
        move |podcast_delete| {
            if podcast_delete.data.is_some() {
                history.push(AppRoute::Home);
            }
            || ()
        },
        podcast_delete,
    );

    if props.can_modify {
        html! {
            <span>
                <Link<AppRoute> to={AppRoute::Editor { slug: props.slug.clone() }} classes="btn btn-outline-secondary btn-sm" >
                    { "Edit Podcast" }
                </Link<AppRoute>>
                { " " }
                <button class="btn btn-outline-danger btn-sm" {onclick} >
                    <i class="ion-trash-a"></i> { "Delete Podcast" }
                </button>
            </span>
        }
    } else {
        html! {
            <span>
            </span>
        }
    }
}
