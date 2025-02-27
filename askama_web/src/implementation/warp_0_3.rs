pub use askama::Template;
use http_0_2::StatusCode;
use warp_0_3::reply::html;
pub use warp_0_3::reply::{Reply, Response};

pub use crate::__askama_web_impl_warp_0_3 as derive;

#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_warp_0_3 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::warp_0_3::derive!($ast where Self: Send);
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: $_impl_generics:tt,
        ty_generics: $_ty_generics:tt,
        where_clause: $_where_clause:tt,
        ex_impl_generics: [$($impl_generics:tt)*],
        ex_ty_generics: [$($ty_generics:tt)*],
        ex_where_clause: [$($where_clause:tt)*],
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::warp_0_3 as __askama_web;

            impl $($impl_generics)* __askama_web::Reply
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[track_caller]
                fn into_response(self) -> __askama_web::Response {
                    __askama_web::into_response(<_ as __askama_web::Template>::render(&self))
                }
            }
        };
    };
}

#[track_caller]
pub fn into_response(result: askama::Result<String>) -> Response {
    match result {
        Ok(body) => html(body).into_response(),
        Err(err) => {
            crate::render_error(&err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
