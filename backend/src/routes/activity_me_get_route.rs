use warp;
use warp::filters::BoxedFilter;
use warp::Filter;

pub fn activity_me_get() -> BoxedFilter<()> {
    warp::get().and(warp::path!("activity" / "me")).boxed()
}
