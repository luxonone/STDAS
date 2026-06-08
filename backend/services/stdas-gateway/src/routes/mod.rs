pub(crate) mod api_v1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteSpec {
    pub method: &'static str,
    pub path: &'static str,
}

const ROUTES: &[RouteSpec] = &[
    RouteSpec {
        method: "POST",
        path: "/api/v1/auth/login",
    },
    RouteSpec {
        method: "GET",
        path: "/api/v1/auth/me",
    },
    RouteSpec {
        method: "GET",
        path: "/api/v1/data/lots",
    },
    RouteSpec {
        method: "GET",
        path: "/api/v1/system/health",
    },
    RouteSpec {
        method: "GET",
        path: "/api/v1/system/preflight",
    },
];

#[must_use]
pub fn route_specs() -> &'static [RouteSpec] {
    ROUTES
}

#[cfg(test)]
mod tests {
    use super::{route_specs, RouteSpec};

    #[test]
    fn route_specs_expose_preflight_contract_paths() {
        assert_eq!(
            route_specs(),
            &[
                RouteSpec {
                    method: "POST",
                    path: "/api/v1/auth/login",
                },
                RouteSpec {
                    method: "GET",
                    path: "/api/v1/auth/me",
                },
                RouteSpec {
                    method: "GET",
                    path: "/api/v1/data/lots",
                },
                RouteSpec {
                    method: "GET",
                    path: "/api/v1/system/health",
                },
                RouteSpec {
                    method: "GET",
                    path: "/api/v1/system/preflight",
                },
            ],
        );
    }
}
