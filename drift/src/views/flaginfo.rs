use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use crate::views::flag::{Flag, FlagInfo};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub flag: String,
}

#[function_component(DisplayFlag)]
pub fn flag(props: &Props) -> Html {
    let flags = use_state(|| vec![]);
    let search = props.flag.clone();
    {
        let flags = flags.clone();
        use_effect_with_deps(move |_| {
            let flags = flags.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let jwt: String = LocalStorage::get("_AuthToken").unwrap();

                let client = reqwest::Client::builder()
                    .build()
                    .unwrap();
                let url = format!("http://127.0.0.1:8000/api/v1/get_flag/{}", search);
                let fetched_flags: Vec<Flag> = client.get(&url)
                    .header("auth", jwt)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                flags.set(fetched_flags);
            });
            || ()
        }, ());
    }
    html! {
        <>
            <div>
                <FlagInfo flag={(*flags).clone()}/>
            </div>
        </>
    }
 
}
