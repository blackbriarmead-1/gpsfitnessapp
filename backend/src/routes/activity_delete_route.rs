use warp;
use warp::filters::BoxedFilter;
use warp::Filter;

pub fn activity_delete() -> BoxedFilter<()> {
    warp::delete().and(warp::path!("activity")).boxed()
}
