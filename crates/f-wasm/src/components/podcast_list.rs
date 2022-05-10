use yew::prelude::*;
use yew_hooks::use_async;

use super::podcast_preview::PodcastPreview;
use super::list_pagination::ListPagination;
use crate::services::podcasts::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub filter: PodcastListFilter,
}

/// Filters for podcast list
#[derive(Clone, Debug, PartialEq)]
pub enum PodcastListFilter {
    All,
    ByAuthor(String),
    ByTag(String),
    FavoritedBy(String),
    Feed,
}

/// List of podcast component
#[function_component(PodcastList)]
pub fn podcast_list(props: &Props) -> Html {
    let current_page = use_state(|| 0u32);
    let podcast_list = {
        let filter = props.filter.clone();
        let current_page = current_page.clone();

        use_async(async move {
            match filter {
                PodcastListFilter::All => all(*current_page).await,
                PodcastListFilter::ByAuthor(author) => by_author(author, *current_page).await,
                PodcastListFilter::ByTag(tag) => by_tag(tag, *current_page).await,
                PodcastListFilter::FavoritedBy(author) => favorited_by(author, *current_page).await,
                PodcastListFilter::Feed => feed().await,
            }
        })
    };

    {
        let current_page = current_page.clone();
        use_effect_with_deps(
            move |_| {
                // Reset to first page
                current_page.set(0);
                || ()
            },
            props.filter.clone(),
        );
    }

    {
        let podcast_list = podcast_list.clone();
        use_effect_with_deps(
            move |_| {
                podcast_list.run();
                || ()
            },
            (props.filter.clone(), *current_page),
        );
    }

    let callback = {
        let current_page = current_page.clone();
        Callback::from(move |page| {
            current_page.set(page);
        })
    };

    if let Some(podcast_list) = &podcast_list.data {
        if !podcast_list.podcasts.is_empty() {
            html! {
                <>
                    {for podcast_list.podcasts.iter().map(|podcast| {
                        html! { <PodcastPreview podcast={podcast.clone()} /> }
                    })}
                    <ListPagination
                        total_count={podcast_list.podcasts_count}
                        current_page={*current_page}
                        callback={callback} />
                </>
            }
        } else {
            html! {
                <div class="podcast-preview">{ "No podcasts are here... yet." }</div>
            }
        }
    } else {
        html! {
            <div class="podcast-preview">{ "Loading..." }</div>
        }
    }
}
