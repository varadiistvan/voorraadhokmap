use axum::{
    body::Body,
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    response::Response,
};
use axum_extra::headers::authorization::{Basic, Credentials};
use futures_util::future::BoxFuture;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthLayer {
    username: String,
    password: String,
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            password: self.password.clone(),
            username: self.username.clone(),
        }
    }
}

impl AuthLayer {
    pub fn new(username: String, password: String) -> Self {
        AuthLayer { username, password }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    password: String,
    username: String,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let username = self.username.clone();
        let password = self.password.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract the Authorization header
            if let Some(auth_header) = req.headers().get(axum::http::header::AUTHORIZATION) {
                if let Some(auth_data) = Basic::decode(auth_header) {
                    // Check username and password
                    if auth_data.username() == username && auth_data.password() == password {
                        // If authentication succeeds, call the inner service
                        return inner.call(req).await;
                    }
                }
            }

            // If authentication fails, return a 401 Unauthorized response
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(
                    axum::http::header::WWW_AUTHENTICATE,
                    r#"Basic realm="Admin Area""#,
                )
                .body(Body::empty())
                .unwrap();
            Ok(response)
        })
    }
}
