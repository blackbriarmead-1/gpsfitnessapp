use warp;
use warp::filters::BoxedFilter;
use warp::Filter;

use crate::json_body;

pub fn activity_delete() -> BoxedFilter<()> {
    warp::delete().and(warp::path!("activity")).boxed()
}
