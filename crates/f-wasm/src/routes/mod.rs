//! Routes by yew_router

pub mod podcast;
pub mod editor;
pub mod home;
pub mod login;
pub mod profile;
pub mod register;
pub mod settings;

use yew::prelude::*;
use yew_router::prelude::*;

use podcast::Podcast;
use editor::Editor;
use home::Home;
use login::Login;
use profile::{Profile, ProfileTab};
use register::Register;
use settings::Settings;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/editor/:slug")]
    Editor { slug: String },
    #[at("/editor")]
    EditorCreate,
    #[at("/podcast/:slug")]
    Podcast { slug: String },
    #[at("/settings")]
    Settings,
    #[at("/:username/favorites")]
    ProfileFavorites { username: String },
    #[at("/:username")]
    Profile { username: String },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::Register => html! {<Register />},
        AppRoute::Home => html! {<Home />},
        AppRoute::Editor { slug } => html! {<Editor slug={Some(slug.clone())}/>},
        AppRoute::EditorCreate => html! {<Editor />},
        AppRoute::Podcast { slug } => html! {<Podcast slug={slug.clone()} />},
        AppRoute::Settings => html! {<Settings />},
        AppRoute::ProfileFavorites { username } => html! {
            <Profile username={username.clone()} tab={ProfileTab::FavoritedBy} />
        },
        AppRoute::Profile { username } => html! {
            <Profile username={username.clone()} tab={ProfileTab::ByAuthor} />
        },
        AppRoute::NotFound => html! { "Page not found" },
    }
}
