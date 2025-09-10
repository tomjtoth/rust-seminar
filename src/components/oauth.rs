use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn OAuthLoginPage() -> Element {
    let handler = move |_| {
        #[cfg(not(feature = "server"))]
        {
            use dioxus_oauth::prelude::{OAuthClient, TokenResponse};
            use std::env::var;

            spawn(async move {
                let client_id = var("CLIENT_ID").expect("CLIENT_ID not set");
                // let client_secret = var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
                let redirect_uri = "/oauth/code".to_string();

                let client = OAuthClient::new(
                    &client_id,
                    &redirect_uri,
                    "https://discord.com/oauth2/authorize",
                    "https://discord.com/api/oauth2/token",
                );
                // .set_openid_url("https://kauth.kakao.com/oauth/tokeninfo");
                // .set_client_secret(&client_secret);

                let code: String = match client.get_auth_code().await {
                    Ok(code) => code,
                    Err(e) => {
                        tracing::error!("Auth code failed: {:?}", e);
                        return;
                    }
                };

                let token_response: TokenResponse = match client.get_token(code.as_str()).await {
                    Ok(token_response) => token_response,
                    Err(e) => {
                        tracing::error!("Token response failed: {:?}", e);
                        return;
                    }
                };
                tracing::debug!("Token response: {:?}", token_response);

                let oid_response: dioxus_oauth::prelude::OpenIdResponse =
                    match client.get_openid(&token_response.id_token).await {
                        Ok(oid_response) => oid_response,
                        Err(e) => {
                            tracing::error!("Token response failed: {:?}", e);
                            return;
                        }
                    };

                tracing::debug!("OID response: {:?}", oid_response);
            });
        }
    };

    rsx! {
        div {
            onclick: handler,
            "faaf login.."
        }
    }
}
