use warp;
use warp::filters::BoxedFilter;
use warp::Filter;

pub fn activity_get() -> BoxedFilter<()> {
    warp::get().and(warp::path!("activity")).boxed()
}
