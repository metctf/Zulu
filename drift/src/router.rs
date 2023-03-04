use gloo_storage::errors::StorageError;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

use crate::views::components::top_bar::{NavBar, Tab};
use crate::views::createflag::{CreateFlag, FlagData};
use crate::views::login::LoginComponent;
use crate::views::home::Home;
use crate::views::register::RegisterComponent;
use crate::views::notfound::NotFound;
use crate::views::settings::modify::ModifyComponent;
use crate::views::challengeinfo::{DisplayChallenge, ChallengeTemplate};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum MainRoute{
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
    #[at("/display_challenge/:id")]
    DisplayChallenge {id:String},
    #[at("/challenge/:id")]
    Challenge {id:String},
    #[at("/create_flag")]
    CreateFlag,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum SettingsRoute{
    #[at("/settings")]
    Settings,
    #[at("/settings/modify")]
    Modify,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}
pub fn switch_main(route: MainRoute) -> Html {
    
    let jwt: Result<String,StorageError>= LocalStorage::get("_AuthToken"); 
    let auth: Tab;

    match jwt {
        Ok(_) => {
                auth = Tab::Authorized;
            },
        Err(_) => {
            auth = Tab::Unauthorized;
        }
    }
    match route {
        MainRoute::Login => {
            html! {
                <LoginComponent />
            }
        },
        MainRoute::Register => {
           html! {
                <RegisterComponent />
           } 
        },
        MainRoute::CreateFlag => {
            let custom_form_submit = Callback::from(|data: FlagData| {
                log!("challenge is", &data.name.to_string());

                wasm_bindgen_futures::spawn_local( async move {
                    let url = format!("http://127.0.0.1:8000/api/v1/create_flag");
                    let form = (
                        ("id", "".to_string()),
                        ("name",data.name),
                        ("author",data.author),
                        ("flag", data.flag),
                        ("points", data.points)
                    );
                    let client = reqwest::Client::new();

                    client.post(&url)
                        .form(&form)
                        .send()
                        .await
                        .unwrap(); //Getting an error here
                });
            });
            html! {
                <>
                    <NavBar tab={Tab::Authorized}/>
                    <CreateFlag onsubmit={custom_form_submit}/>
                </>
            }
        },
       MainRoute::Home => {
            
            html! {
            <>
                <NavBar tab={auth}/>
                <Home />
            </>
            }
        },
        MainRoute::DisplayChallenge { id } => {
            html! {
            <>
                <NavBar tab={auth}/>
                <DisplayChallenge flag={id}/>
            </>
            }
        },
        MainRoute::Challenge { id } => {
            html! {
                <>
                    <NavBar tab={auth}/>
                    <ChallengeTemplate flag={id} />
                </>
            }
        }
        MainRoute::SettingsRoot | MainRoute::Settings => html! { <Switch<SettingsRoute> render={switch_settings} /> },
        MainRoute::NotFound => {
            html! {
            <>
                <NavBar tab={auth}/>
                <NotFound />
            </>
            }
        },
    }
}

pub fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Settings =>  html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>},
        SettingsRoute::Modify => {
            html!(<ModifyComponent />)
       },
        SettingsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>}
    }
}

