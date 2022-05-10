use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::podcasts::*;
use crate::types::PodcastInfo;

const FAVORITED_CLASS: &str = "btn btn-sm btn-primary";
const NOT_FAVORITED_CLASS: &str = "btn btn-sm btn-outline-primary";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub podcast: PodcastInfo,
}

/// Single podcast preview component used by podcast list.
#[function_component(PodcastPreview)]
pub fn podcast_preview(props: &Props) -> Html {
    let podcast = use_state(|| props.podcast.clone());
    let podcast_favorite = {
        let podcast = podcast.clone();
        use_async(async move {
            if podcast.favorited {
                unfavorite(podcast.slug.clone()).await
            } else {
                favorite(podcast.slug.clone()).await
            }
        })
    };

    {
        let podcast = podcast.clone();
        let podcast_favorite = podcast_favorite.clone();
        use_effect_with_deps(
            move |podcast_favorite| {
                if let Some(podcast_info) = &podcast_favorite.data {
                    podcast.set(podcast_info.podcast.clone());
                }
                || ()
            },
            podcast_favorite,
        );
    }

    let favorite_button_class = if podcast.favorited {
        FAVORITED_CLASS
    } else {
        NOT_FAVORITED_CLASS
    };
    let onclick = {
        Callback::from(move |ev: MouseEvent| {
            ev.prevent_default();
            podcast_favorite.run();
        })
    };

    html! {
        <div class="podcast-preview">
            <div class="podcast-meta">
                <img src={podcast.author.image.clone()} />
                <div class="info">
                    <Link<AppRoute>
                        to={AppRoute::Profile { username: podcast.author.username.clone() }}
                        classes="author" >
                        { &podcast.author.username }
                    </Link<AppRoute>>
                    <span class="date">
                        { &podcast.created_at.format("%B %e, %Y") }
                    </span>
                </div>
                <div class="pull-xs-right">
                    <button class={favorite_button_class} onclick={onclick}>
                        <i class="ion-heart"></i> { podcast.favorites_count }
                    </button>
                </div>
            </div>
            <h1>
                <Link<AppRoute>
                    to={AppRoute::Podcast { slug: podcast.slug.clone() }}
                    classes="preview-link" >
                { &podcast.title }
                </Link<AppRoute>>
            </h1>
            <p>{ &podcast.description }</p>
            <span>
                <Link<AppRoute>
                    to={AppRoute::Podcast { slug: podcast.slug.clone() }}
                    classes="preview-link" >
                    { "Read more..." }
                </Link<AppRoute>>
            </span>
            <ul class="tag-list">
                {for podcast.tag_list.iter().map(|tag| {
                    html! {
                        <li class="tag-default tag-pill tag-outline" key={ (&tag).to_string() }>
                            { &tag }
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
