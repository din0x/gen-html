#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
mod axum {
    use crate::Raw;
    use axum::response::{Html, IntoResponse, Response};

    impl<T: IntoResponse> IntoResponse for Raw<T> {
        fn into_response(self) -> Response {
            Html(self.0).into_response()
        }
    }
}

#[cfg(feature = "actix-web")]
#[cfg_attr(docsrs, doc(cfg(feature = "actix-web")))]
mod actix_web {
    use crate::Raw;
    use actix_web::{
        HttpRequest, HttpResponse, Responder,
        body::EitherBody,
        http::header::{self, ContentType},
    };

    impl<T: Responder> Responder for Raw<T> {
        type Body = EitherBody<T::Body>;

        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            self.0
                .customize()
                .insert_header((header::CONTENT_TYPE, ContentType::html()))
                .respond_to(req)
        }
    }
}
