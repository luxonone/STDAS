use crate::services::SystemService;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    system_service: SystemService,
}

impl AppState {
    pub(crate) fn system_service(&self) -> &SystemService {
        &self.system_service
    }
}
