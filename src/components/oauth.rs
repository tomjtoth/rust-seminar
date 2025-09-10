use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn OAuthLoginPage() -> Element {
    rsx! {
       div {
           onclick: move |_| {
              #[cfg(not(feature = "server"))]
              {
                  spawn(async move {
                      let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID not set");
                      let redirect_uri = std::env::var("REDIRECT_URI").expect("REDIRECT_URI not set");
                      let client = dioxus_oauth::prelude::OAuthClient::new(
                          &client_id,
                          &redirect_uri,
                          "https://kauth.kakao.com/oauth/authorize",
                          "https://kauth.kakao.com/oauth/token",
                      )
                      .set_openid_url("https://kauth.kakao.com/oauth/tokeninfo");

                      let code: String = match client.get_auth_code().await {
                          Ok(code) => code,
                          Err(e) => {
                              tracing::error!("Auth code failed: {:?}", e);
                              return;
                          }
                      };

                      let token_response: dioxus_oauth::prelude::TokenResponse =
                          match client.get_token(code.as_str()).await {
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
           }
       }
    }
}
