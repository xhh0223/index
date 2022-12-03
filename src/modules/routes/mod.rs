use axum::Router;
mod index;

/* 存储所有的路由 */
static mut ROUTER_SERVICE_CENTER: Option<&'static mut RouterServiceCenter> = None;

pub struct RouterServiceCenter {
    services: Vec<Router>,
}

impl RouterServiceCenter {
    fn new() -> &'static mut Option<&'static mut RouterServiceCenter> {
        unsafe {
            let temp_value = Box::new(RouterServiceCenter { services: vec![] });
            ROUTER_SERVICE_CENTER = Some(Box::leak(temp_value));
            &mut ROUTER_SERVICE_CENTER
        }
    }

    pub fn get_single_instance() -> &'static mut Option<&'static mut RouterServiceCenter> {
        unsafe {
            if matches!(ROUTER_SERVICE_CENTER, Some(_)) {
                &mut ROUTER_SERVICE_CENTER
            } else {
                RouterServiceCenter::new()
            }
        }
    }

    pub fn get_app(&mut self) -> Router {
        self::index::register();
        let mut app = Router::new();
        while let Some(r) = self.services.pop() {
            app = app.merge(r)
        }
        print!("{:?}", self.services);
        app
    }

    pub fn register_router(&mut self, router_service: Router) {
        self.services.push(router_service)
    }
}
