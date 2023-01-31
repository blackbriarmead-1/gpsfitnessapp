use warp;
use warp::filters::BoxedFilter;
use warp::Filter;

pub fn activity_post() -> BoxedFilter<()> {
    warp::post().and(warp::path!("activity")).boxed()
}
