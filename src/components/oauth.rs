use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn OAuthLoginPage() -> Element {
    let handler = move |_| {
        #[cfg(not(feature = "server"))]
        {
            use dioxus_oauth::prelude::{OAuthClient, TokenResponse};

            spawn(async move {
                let client_id = option_env!("CLIENT_ID").expect("CLIENT_ID not set");
                let redirect_uri = format!("{}/oauth/code", crate::utils::server_url());

                let client = OAuthClient::new(
                    &client_id,
                    &redirect_uri,
                    "https://discord.com/oauth2/authorize",
                    "https://discord.com/api/oauth2/token",
                )
                .add_scope("email");

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
        button {
            onclick: handler,
            "login via Discord"
        }
    }
}
