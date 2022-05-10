use yew::{function_component, html};
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="logo-font">{ "cryptcast" }</Link<AppRoute>>
                <span class="attribution">
                    { "© 2022. A decentralized podcast platform from" }
                    <a href="https://"> { "cryptcast" } </a>
                    { ". Code licensed under MPL-2.0." }
                </span>
            </div>
        </footer>
    }
}
