use warp::Filter;

use crate::app::AppState;
use crate::handlers;

pub fn routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(handlers::index::index)
        .or(warp::path!("api" / "user")
            .and(warp::get())
            .and(warp::header("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::users::get_current_user))
        .or(warp::path!("api" / "user")
            .and(warp::put())
            .and(warp::header("Authorization"))
            .and(warp::body::json())
            .and(with_state(state.clone()))
            .map_async(handlers::users::update_user))
        .or(warp::path!("api" / "users")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_state(state.clone()))
            .map_async(handlers::users::register))
        .or(warp::path!("api" / "users" / "login")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_state(state.clone()))
            .map_async(handlers::users::login))
        .or(warp::path!("api" / "profiles" / String)
            .and(warp::get())
            .and(warp::header::optional("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::profiles::get_profile))
        .or(warp::path!("api" / "profiles" / String / "follow")
            .and(warp::post())
            .and(warp::header("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::profiles::follow))
        .or(warp::path!("api" / "profiles" / String / "follow")
            .and(warp::delete())
            .and(warp::header("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::profiles::unfollow))
        .or(warp::path!("api" / "articles")
            .and(warp::get())
            .and(warp::header::optional("Authorization"))
            .and(warp::query())
            .and(with_state(state.clone()))
            .map_async(handlers::articles::list_articles))
        .or(warp::path!("api" / "articles")
            .and(warp::post())
            .and(warp::header("Authorization"))
            .and(warp::body::json())
            .and(with_state(state.clone()))
            .map_async(handlers::articles::insert_article))
        .or(warp::path!("api" / "articles" / "feed")
            .and(warp::get())
            .and(warp::header("Authorization"))
            .and(warp::query())
            .and(with_state(state.clone()))
            .map_async(handlers::articles::feed))
        .or(warp::path!("api" / "articles" / String)
            .and(warp::get())
            .and(warp::header::optional("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::articles::get_article))
        .or(warp::path!("api" / "articles" / String)
            .and(warp::put())
            .and(warp::header("Authorization"))
            .and(warp::body::json())
            .and(with_state(state.clone()))
            .map_async(handlers::articles::update_article))
        .or(warp::path!("api" / "articles" / String)
            .and(warp::delete())
            .and(warp::header("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::articles::delete_article))
        .or(warp::path!("api" / "articles" / String / "favorite")
            .and(warp::post())
            .and(warp::header("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::articles::favorite))
        .or(warp::path!("api" / "articles" / String / "favorite")
            .and(warp::delete())
            .and(warp::header("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::articles::unfavorite))
        .or(warp::path!("api" / "articles" / String / "comments")
            .and(warp::get())
            .and(warp::header::optional("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::comments::get))
        .or(warp::path!("api" / "articles" / String / "comments")
            .and(warp::post())
            .and(warp::header("Authorization"))
            .and(warp::body::json())
            .and(with_state(state.clone()))
            .map_async(handlers::comments::create))
        .or(warp::path!("api" / "articles" / String / "comments" / u64)
            .and(warp::delete())
            .and(warp::header("Authorization"))
            .and(with_state(state.clone()))
            .map_async(handlers::comments::delete))
        .or(warp::path!("api" / "tags")
            .and(warp::get())
            .and(with_state(state.clone()))
            .map_async(handlers::articles::tags))
}

fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
