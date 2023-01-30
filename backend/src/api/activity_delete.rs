#[macro_export]
macro_rules! activity_delete {
    () => {
        activity_delete_route::activity_delete()
            .and(json_body!(OIDS))
            .and_then(activity_delete_handler::activity_delete)
    };
}
