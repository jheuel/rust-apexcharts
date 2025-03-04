#[cfg(not(feature = "ssr"))]
mod csr {
    use serde_json::Value;
    use std::error::Error;
    use wasm_bindgen::prelude::*;
    use web_sys::Element;

    use crate::options::{ChartSeries, ChartType, SeriesData};

    #[wasm_bindgen(module = "/node_modules/apexcharts/dist/apexcharts.esm.js")]
    extern "C" {
        #[wasm_bindgen(js_name = default)]
        pub type ApexCharts;

        #[wasm_bindgen(constructor, js_class = default)]
        pub fn new(element: &Element, options: &JsValue) -> ApexCharts;

        #[wasm_bindgen(method, js_class = default)]
        pub fn render(this: &ApexCharts);

        #[wasm_bindgen(method, js_class = default)]
        pub fn destroy(this: &ApexCharts);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = JSON)]
        fn parse(json: &str) -> JsValue;
    }

    pub struct ApexChart {
        chart: ApexCharts,
    }

    impl ApexChart {
        pub fn new(
            element_id: &str,
            chart_type: ChartType,
            series: Vec<ChartSeries>,
            options: &str,
        ) -> Result<ApexChart, Box<dyn Error>> {
            let mut labels_data = None;
            let series_data = match chart_type {
                ChartType::Pie | ChartType::Donut | ChartType::RadialBar => {
                    let chart_series = series.clone();
                    match chart_series.first() {
                        Some(chart_serie) => match chart_serie.data {
                            SeriesData::Radial(ref data) => {
                                let data_values = data.iter().map(|(_, y)| *y).collect::<Vec<_>>();
                                labels_data =
                                    Some(data.iter().map(|(x, _)| x.clone()).collect::<Vec<_>>());
                                serde_json::to_value(data_values).unwrap_or(Value::Array(vec![]))
                            }
                            _ => serde_json::to_value(series).unwrap_or(Value::Array(vec![])),
                        },
                        _ => Value::Array(vec![]),
                    }
                }
                _ => serde_json::to_value(series).unwrap_or(Value::Array(vec![])),
            };

            let width = "100%".to_string();
            let height = "auto".to_string();
            let options = if options.is_empty() {
                format!(
                    r#"{{
                        "chart": {{
                            "type": "{}",
                            "width": "{}",
                            "height": "{}"
                        }},
                        "series": {}
                        {}
                    }}"#,
                    chart_type,
                    width,
                    height,
                    series_data,
                    if let Some(labels) = labels_data {
                        format!(
                            r#","labels": {}"#,
                            serde_json::to_string(&labels).unwrap_or("[]".to_string())
                        )
                    } else {
                        "".to_string()
                    }
                )
            } else {
                let mut options = serde_json::from_str::<Value>(&options)
                    .unwrap_or_else(|_| panic!("Invalid JSON: {}", options));
                options["chart"]["type"] = Value::String(chart_type.to_string());
                options["chart"]["width"] = Value::String(width.clone());
                options["chart"]["height"] = Value::String(height.clone());
                options["series"] = series_data;
                if let Some(labels) = labels_data {
                    options["labels"] = Value::Array(
                        labels
                            .iter()
                            .map(|label| Value::String(label.clone()))
                            .collect(),
                    );
                }
                serde_json::to_string(&options).unwrap_or_default()
            };

            let window = web_sys::window().ok_or("No window")?;
            let document = window.document().ok_or("No document")?;
            let element = document
                .get_element_by_id(element_id)
                .ok_or("Element not found")?;
            let chart = ApexCharts::new(&element, &parse(&options));
            Ok(ApexChart { chart })
        }

        pub fn render(&self) {
            self.chart.render();
        }

        pub fn destroy(&self) {
            self.chart.destroy();
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use std::error::Error;

    use crate::options::{ChartSeries, ChartType};

    pub struct ApexChart;

    impl ApexChart {
        pub fn new(
            _element_id: &str,
            _chart_type: ChartType,
            _series: Vec<ChartSeries>,
            _options: &str,
        ) -> Result<Self, Box<dyn Error>> {
            Ok(Self)
        }

        pub fn render(&self) {}

        pub fn destroy(&self) {}
    }
}

#[cfg(not(feature = "ssr"))]
pub use csr::*;
#[cfg(feature = "ssr")]
pub use ssr::*;
