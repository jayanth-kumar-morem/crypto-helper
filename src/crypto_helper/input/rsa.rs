use web_sys::{Event, HtmlInputElement};
use yew::{classes, function_component, html, Callback, Classes, Html, Properties, TargetCast};

use crate::crypto_helper::algorithm::{
    RsaAction, RsaHashAlgorithm, RsaInput as RsaInputData, RsaSignInput, RsaVerifyInput, RSA_HASH_ALGOS,
};

#[derive(Debug, PartialEq, Properties)]
pub struct RsaInputProps {
    pub input: RsaInputData,
    pub setter: Callback<RsaInputData>,
}

fn get_action_classes(is_selected: bool) -> Classes {
    if is_selected {
        classes!("rsa-action", "selected-rsa-action")
    } else {
        classes!("rsa-action")
    }
}

fn generate_selection_action_component(action: &RsaAction, set_action: Callback<RsaAction>) -> Html {
    html! {
        <div class={classes!("rsa-actions-container")}>
            {RsaAction::enumerate_actions()
                .iter()
                .map(|action_name| {
                    let set_action = set_action.clone();
                    html! {
                        <span
                            class={get_action_classes(action == action_name)}
                            onclick={Callback::from(move |_| set_action.emit((*action_name).try_into().unwrap_or_default()))}
                        >
                            {action_name}
                        </span>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}

fn get_hash_selection_component(hash_algorithm: &RsaHashAlgorithm, set_hash_algo: Callback<RsaHashAlgorithm>) -> Html {
    let onchange = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(hash_algorithm) = input.value().as_str().try_into() {
            log::info!("set new rsa hash algorithm: {:?}", hash_algorithm);
            set_hash_algo.emit(hash_algorithm);
        }
    });

    html! {
        <select class={classes!("base-input", "auto-width-input")} {onchange}>
            {RSA_HASH_ALGOS
                .iter()
                .map(|hash_algo_name| {
                    html! {
                        <option
                            selected={hash_algorithm == hash_algo_name}
                            value={hash_algo_name.to_string()}
                        >
                            {hash_algo_name}
                        </option>
                    }
                })
                .collect::<Vec<_>>()}
        </select>
    }
}

fn generate_rsa_input(input: &RsaAction, set_action: Callback<RsaAction>) -> Html {
    let selected_algorithm_component = generate_selection_action_component(input, set_action.clone());
    match input {
        RsaAction::Encrypt(input) => {
            let oninput = Callback::from(move |event: html::oninput::Event| {
                let input: HtmlInputElement = event.target_unchecked_into();
                set_action.emit(RsaAction::Encrypt(input.value()));
            });

            html! {
                <div class={classes!("vertical")}>
                    {selected_algorithm_component}
                    <textarea
                        rows="4"
                        placeholder={"RSA public key in PEM (-----BEGIN RSA PUBLIC KEY-----)"}
                        class={classes!("base-input")}
                        value={input.clone()}
                        {oninput}
                    />
                </div>
            }
        }
        RsaAction::Decrypt(input) => {
            let oninput = Callback::from(move |event: html::oninput::Event| {
                let input: HtmlInputElement = event.target_unchecked_into();
                set_action.emit(RsaAction::Decrypt(input.value()));
            });

            html! {
                <div class={classes!("vertical")}>
                    {selected_algorithm_component}
                    <textarea
                        rows="4"
                        placeholder={"RSA private key in PEM (-----BEGIN RSA PRIVATE KEY-----)"}
                        class={classes!("base-input")}
                        value={input.clone()}
                        {oninput}
                    />
                </div>
            }
        }
        RsaAction::Sign(input) => {
            let set_action_algo = set_action.clone();
            let rsa_key = input.rsa_key.clone();
            let set_hash_algo = Callback::from(move |hash_algorithm| {
                set_action_algo.emit(RsaAction::Sign(RsaSignInput {
                    hash_algorithm,
                    rsa_key: rsa_key.clone(),
                }));
            });

            let hash_algorithm = input.hash_algorithm;
            let on_rsa_key_input = Callback::from(move |event: html::oninput::Event| {
                let input: HtmlInputElement = event.target_unchecked_into();
                set_action.emit(RsaAction::Sign(RsaSignInput {
                    hash_algorithm,
                    rsa_key: input.value(),
                }));
            });

            html! {
                <div class={classes!("vertical")}>
                    {selected_algorithm_component}
                    <div class={classes!("horizontal")}>
                        {get_hash_selection_component(&input.hash_algorithm, set_hash_algo)}
                        <textarea
                            rows="4"
                            placeholder={"RSA private key in PEM (-----BEGIN RSA PRIVATE KEY-----)"}
                            class={classes!("base-input")}
                            value={input.rsa_key.clone()}
                            oninput={on_rsa_key_input}
                        />
                    </div>
                </div>
            }
        }
        RsaAction::Verify(input) => {
            let set_action_algo = set_action.clone();
            let rsa_key = input.rsa_key.clone();
            let signature = input.signature.clone();
            let set_hash_algo = Callback::from(move |hash_algorithm| {
                set_action_algo.emit(RsaAction::Verify(RsaVerifyInput {
                    hash_algorithm,
                    rsa_key: rsa_key.clone(),
                    signature: signature.clone(),
                }));
            });

            let set_action_key = set_action.clone();
            let hash_algorithm = input.hash_algorithm;
            let signature = input.signature.clone();
            let on_rsa_key_input = Callback::from(move |event: html::oninput::Event| {
                let input: HtmlInputElement = event.target_unchecked_into();
                set_action_key.emit(RsaAction::Verify(RsaVerifyInput {
                    hash_algorithm,
                    rsa_key: input.value(),
                    signature: signature.clone(),
                }));
            });

            let hash_algorithm = input.hash_algorithm;
            let rsa_key = input.rsa_key.clone();
            let on_signature_input = Callback::from(move |event: html::oninput::Event| {
                let input: HtmlInputElement = event.target_unchecked_into();
                set_action.emit(RsaAction::Verify(RsaVerifyInput {
                    hash_algorithm,
                    rsa_key: rsa_key.clone(),
                    signature: input.value(),
                }));
            });

            html! {
                <div class={classes!("vertical")}>
                    {selected_algorithm_component}
                    <div class={classes!("horizontal")}>
                        {get_hash_selection_component(&input.hash_algorithm, set_hash_algo)}
                        <textarea
                            rows="4"
                            placeholder={"RSA public key in PEM (-----BEGIN RSA PUBLIC KEY-----)"}
                            class={classes!("base-input")}
                            value={input.rsa_key.clone()}
                            oninput={on_rsa_key_input}
                        />
                    </div>
                    <textarea
                        rows="3"
                        placeholder={"hex-encoded signature"}
                        class={classes!("base-input")}
                        value={input.signature.clone()}
                        oninput={on_signature_input}
                    />
                </div>
            }
        }
    }
}

#[function_component(RsaInput)]
pub fn rsa_input(props: &RsaInputProps) -> Html {
    let setter = props.setter.clone();
    let payload = props.input.payload.clone();
    let set_action = Callback::from(move |action| {
        setter.emit(RsaInputData {
            action,
            payload: payload.clone(),
        });
    });

    let setter = props.setter.clone();
    let action = props.input.action.clone();
    let oninput = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        setter.emit(RsaInputData {
            action: action.clone(),
            payload: input.value(),
        });
    });

    html! {
        <div class={classes!("vertical")}>
            {generate_rsa_input(&props.input.action, set_action)}
            <textarea
                rows="2"
                placeholder={"hex-encoded input"}
                class={classes!("base-input")}
                value={props.input.payload.clone()}
                {oninput}
            />
        </div>
    }
}

pub fn build_rsa_input(input: RsaInputData, setter: Callback<RsaInputData>) -> Html {
    html! {
        <RsaInput input={input} setter={setter} />
    }
}
