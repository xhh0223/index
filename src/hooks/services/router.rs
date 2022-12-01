use axum::Router;

static mut router_service_center: Option<&mut RouterServiceCenter> = None;
pub struct RouterServiceCenter {
    services: Vec<Router>,
}

impl RouterServiceCenter {
    fn new() -> Option<&'static mut RouterServiceCenter> {
        let result = Box::new(RouterServiceCenter { services: vec![] });
        unsafe {
            router_service_center = Some(Box::leak(result));
            router_service_center
        }
    }
    pub fn get_single_instance() -> Option<&'static mut RouterServiceCenter> {
        unsafe {
            if matches!(router_service_center, Some(_)) {
                router_service_center
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
