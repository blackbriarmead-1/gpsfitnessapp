#[macro_export]
macro_rules! activity_post {
    () => {
        activity_post_route::activity_post()
            .and(json_body!(Activity))
            .and_then(activity_post_handler::activity_post)
    };
}
