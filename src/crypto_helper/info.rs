mod algo_search;

use web_sys::HtmlInputElement;
use yew::html::onchange::Event;
use yew::{classes, function_component, html, use_state, Callback, Html, Properties, TargetCast, UseStateSetter};

use super::algorithm::{Algorithm, SUPPORTED_ALGORITHMS};
use crate::crypto_helper::info::algo_search::AlgoSearch;

#[derive(PartialEq, Properties)]
pub struct InfoProps {
    pub algorithm: Algorithm,
    pub set_algorithm: UseStateSetter<Algorithm>,
}

fn get_algorithm_info(algorithm: &Algorithm) -> Html {
    match algorithm {
        Algorithm::Md5(_) => html! {
            <span>
            {"Hash hex-encoded input using MD5 hashing function. "}
            <a href={"https://www.ietf.org/rfc/rfc1321.txt"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Sha1(_) => html! {
            <span>{"Hash hex-encoded input using SHA1 hashing function."}
            <a href={"https://www.rfc-editor.org/rfc/rfc3174"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Sha256(_) => html! {
            <span>{"Hash hex-encoded input using SHA256 hashing function."}
            <a href={"https://www.rfc-editor.org/rfc/rfc4634"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Sha512(_) => html! {
            <span>{"Hash hex-encoded input using SHA512 hashing function."}
            <a href={"https://www.rfc-editor.org/rfc/rfc4634"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Aes128CtsHmacSha196(_) => html! {
            <span>{"Encrypt hex-encoded data with the provided or derived key using AES128-CTS-HMAC-SHA1-96 algorithm."}
            <a href={"https://www.rfc-editor.org/rfc/rfc3961.html"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Aes256CtsHmacSha196(_) => html! {
            <span>{"Encrypt hex-encoded data with the provided or derived key using AES256-CTS-HMAC-SHA1-96 algorithm."}
            <a href={"https://www.rfc-editor.org/rfc/rfc3961.html"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::HmacSha196Aes128(_) => html! {
            <span>{"Hmac with the provided or derived key using HMAC-SHA1-96-AES128 algorithm."}
            <a href={"https://www.rfc-editor.org/rfc/rfc3961.html"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::HmacSha196Aes256(_) => html! {
            <span>{"Hmac with the provided or derived key using HMAC-SHA1-96-AES256 algorithm."}
            <a href={"https://www.rfc-editor.org/rfc/rfc3961.html"}>{"RFC"}</a>{"."}
            </span>
        },
        Algorithm::Rsa(_) => html! {
            <span>{"Use RSA algorithm to sign/verify/encrypt/decrypt your data."}</span>
        },
    }
}

fn get_search_icon() -> Html {
    html! {
        <svg xmlns={"http://www.w3.org/2000/svg"} viewBox={"0 0 64 64"} width={"64px"} height={"64px"}>
            <g id={"surface31746192"}>
                <path
                    style={" stroke:none;fill-rule:nonzero;fill:rgb(25.098041%,21.568628%,20.784314%);fill-opacity:1;"}
                    d={"M 24 2.890625 C 12.367188 2.890625 2.890625 12.367188 2.890625 24 C 2.890625 35.632812 12.367188 45.109375 24 45.109375 C 29.035156 45.109375 33.664062 43.332031 37.296875 40.371094 L 52.128906 58.953125 C 52.128906 58.953125 55.378906 59.4375 57.394531 57.332031 C 59.425781 55.214844 58.949219 52.132812 58.949219 52.132812 L 40.371094 37.296875 C 43.332031 33.664062 45.109375 29.035156 45.109375 24 C 45.109375 12.367188 35.632812 2.890625 24 2.890625 Z M 24 7.109375 C 33.351562 7.109375 40.890625 14.648438 40.890625 24 C 40.890625 33.351562 33.351562 40.890625 24 40.890625 C 14.648438 40.890625 7.109375 33.351562 7.109375 24 C 7.109375 14.648438 14.648438 7.109375 24 7.109375 Z M 24 7.109375 "}
                />
            </g>
        </svg>
    }
}

#[function_component(Info)]
pub fn info(props: &InfoProps) -> Html {
    log::debug!("props algo: {:?}", props.algorithm);

    let algo_search = use_state(|| false);

    let set_algorithm = props.set_algorithm.clone();
    let onchange = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(algorithm) = input.value().as_str().try_into() {
            log::info!("set new algorithm: {:?}", algorithm);
            set_algorithm.set(algorithm);
        }
    });

    let algo_search_value = *algo_search;
    let algo_search_setter = algo_search.setter();
    let on_algo_search_change = Callback::from(move |_| {
        algo_search_setter.set(!algo_search_value);
    });

    html! {
        <div class={classes!("horizontal")}>
            <div class={classes!("vertical")}>
                <div class={classes!("horizontal")}>
                    <select {onchange} class={classes!("base-input")}>{
                        SUPPORTED_ALGORITHMS
                            .iter()
                            .map(|algo| {
                                log::trace!("selected: {}, value: {}", &props.algorithm == *algo, *algo);
                                html!{
                                    <option selected={ &props.algorithm == *algo } value={*algo}>{algo}</option>
                                }
                            })
                            .collect::<Vec<_>>()
                    }</select>
                    <input type="checkbox" id={"algo-search"} class={classes!("search-input")} onchange={on_algo_search_change} />
                    <label for={"algo-search"} class={classes!("search-button")}>
                        {get_search_icon()}
                    </label>
                </div>
                {
                    if *algo_search {
                        html! {
                            <AlgoSearch set_algorithm={props.set_algorithm.clone()} />
                        }
                    } else {
                        html!{ }
                    }
                }
            </div>
            <div class={classes!("algo-info")}>{get_algorithm_info(&props.algorithm)}</div>
        </div>
    }
}
