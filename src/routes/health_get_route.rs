use warp;
use warp::filters::BoxedFilter;
use warp::Filter;

pub fn health_get() -> BoxedFilter<()> {
    warp::get().and(warp::path!("health")).boxed()
}
