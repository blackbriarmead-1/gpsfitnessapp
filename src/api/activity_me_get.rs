#[macro_export]
macro_rules! activity_me_get {
    () => {
        activity_me_get_route::activity_me_get()
            .and(json_body!(ActivityMeOptions))
            .and_then(activity_me_get_handler::activity_me_get)
    };
}
