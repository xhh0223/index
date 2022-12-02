use axum::Router;
static mut ROUTER_SERVICE_CENTER: Option<&'static mut RouterServiceCenter> = None;

pub struct RouterServiceCenter {
    services: Vec<Router>,
}

impl RouterServiceCenter {
    fn new() -> &'static Option<&'static mut RouterServiceCenter> {
        unsafe {
            let temp_value = Box::new(RouterServiceCenter { services: vec![] });
            ROUTER_SERVICE_CENTER = Some(Box::leak(temp_value));
            &ROUTER_SERVICE_CENTER
        }
    }
    pub fn get_single_instance() -> &'static Option<&'static mut RouterServiceCenter> {
        unsafe {
            if matches!(ROUTER_SERVICE_CENTER, Some(_)) {
                &ROUTER_SERVICE_CENTER
            } else {
                RouterServiceCenter::new()
            }
        }
    }

    pub fn services(&mut self) -> &mut Vec<Router> {
        &mut self.services
    }
    pub fn insert(&mut self, router_service: Router) {
        self.services.push(router_service)
    }
}
