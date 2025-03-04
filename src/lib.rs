mod apexcharts;
mod options;

pub use apexcharts::*;

#[cfg(feature = "ssr")]
mod ssr {
    pub fn apexcharts_js_assets() -> Vec<(&'static str, &'static str)> {
        vec![(
            "/node_modules/apexcharts/dist/apexcharts.esm.js",
            include_str!("../node_modules/apexcharts/dist/apexcharts.esm.js"),
        )]
    }

    #[cfg(feature = "axum")]
    pub fn apexcharts_js_router() -> axum::Router {
        use axum::{http::HeaderMap, routing::get, Router};

        let headers = {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "application/javascript".parse().unwrap());
            headers
        };
        let mut router = Router::new();
        for (path, content) in apexcharts_js_assets() {
            router = router.route(path, get((headers.clone(), content)))
        }
        router
    }
}

pub mod prelude {
    pub use crate::apexcharts::ApexChart;
    pub use crate::options::{to_jsvalue, ChartSeries, ChartType, SeriesData};

    #[cfg(feature = "ssr")]
    pub use crate::ssr::*;
}
