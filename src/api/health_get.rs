#[macro_export]
macro_rules! health_get {
    () => {
        health_get_route::health_get().and_then(health_get_handler::health_get)
    };
}
