#[macro_export]
macro_rules! activity_get {
    () => {
        activity_get_route::activity_get()
            .and(json_body!(Activity))
            .and_then(activity_get_handler::activity_get)
    };
}
