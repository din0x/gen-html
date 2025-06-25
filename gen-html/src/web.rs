#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
mod axum {
    use crate::{Escaped, Raw, Render, RenderFn};
    use axum::response::{Html, IntoResponse, Response};
    use std::fmt;

    impl<T: fmt::Display> IntoResponse for Escaped<T> {
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }

    impl<T: fmt::Display> IntoResponse for Raw<T> {
        fn into_response(self) -> Response {
            Html(self.0.to_string()).into_response()
        }
    }

    impl<F> IntoResponse for RenderFn<F>
    where
        F: Fn(&mut fmt::Formatter) -> fmt::Result,
    {
        fn into_response(self) -> Response {
            self.render().into_response()
        }
    }
}

#[cfg(feature = "actix-web")]
#[cfg_attr(docsrs, doc(cfg(feature = "actix-web")))]
mod actix_web {
    use crate::{Escaped, Raw, Render, RenderFn};
    use actix_web::{HttpRequest, HttpResponse, Responder, web::Html};
    use std::fmt;

    impl<T: fmt::Display> Responder for Escaped<T> {
        type Body = String;

        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            self.render().respond_to(req)
        }
    }

    impl<T: fmt::Display> Responder for Raw<T> {
        type Body = String;

        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            Html::new(self.0.to_string()).respond_to(req)
        }
    }

    impl<F> Responder for RenderFn<F>
    where
        F: Fn(&mut fmt::Formatter) -> fmt::Result,
    {
        type Body = String;

        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            self.render().respond_to(req)
        }
    }
}
