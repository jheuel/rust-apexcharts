# rust-apexcharts

Web framework agnostic rust wasm bindings for [apexcharts](https://apexcharts.com/). Inspired by [apexcharts-rs](https://github.com/clementwanjau/apexcharts-rs).

## Usage

#### web-sys
``` rust
use apexcharts::prelude::{ApexChart, ChartSeries, ChartType, SeriesData};
use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mut val = document.create_element("div")?;
    let id = "plot";
    val.set_id(&id);

    let series = vec![ChartSeries {
        name: "Example".to_string(),
        data: SeriesData::Single(vec![100, 150, 160, 180, 300, 1200, 2000, 5000]),
        color: "#1A56DB".to_string(),
        chart_type: None,
        z_index: None,
    }];

    let raw_options = r##"{
        "chart": {
        "background": "0",
        "fontFamily": "Inter, sans-serif",
        "dropShadow": {
            "enabled": false
        },
        "toolbar": {
            "show": false
        }
        },
        "xaxis": {
        "categories": [
            "01 February",
            "02 February",
            "03 February",
            "04 February",
            "05 February",
            "06 February",
            "07 February"
        ],
        "labels": {
            "show": false
        },
        "axisBorder": {
            "show": false
        },
        "axisTicks": {
            "show": false
        }
        },
        "yaxis": {
            "show": false
        },
        "legend": {
            "show": false
        },
        "stroke": {
        "width": 6,
        "curve": "smooth"
        },
        "grid": {
        "show": false,
        "strokeDashArray": 4,
        "padding": {
            "left": 2,
            "right": 2,
            "top": 0
        }
        },
        "dataLabels": {
        "enabled": false
        },
        "tooltip": {
        "enabled": true,
        "x": {
            "show": false
        }
        }
    }"##;

    let chart = ApexChart::new(&id, ChartType::Area, series, raw_options)
        .expect("Could not create chart");

    chart.render();
    body.append_child(&val)?;
    Ok(())
}
```

#### Leptos
``` rust
#[component]
pub fn Plot() -> impl IntoView {
    let el = NodeRef::<html::Div>::new();

        use apexcharts::prelude::{ApexChart, ChartSeries, ChartType, SeriesData};

        let series = vec![ChartSeries {
            name: "Example".to_string(),
            data: SeriesData::Single(vec![100, 150, 160, 180, 300, 1200, 2000, 5000]),
            color: "#1A56DB".to_string(),
            chart_type: None,
            z_index: None,
        }];

        let raw_options = r##"{
          "chart": {
            "background": "0",
            "fontFamily": "Inter, sans-serif",
            "dropShadow": {
              "enabled": false
            },
            "toolbar": {
              "show": false
            }
          },
          "xaxis": {
            "categories": [
                "01 February",
                "02 February",
                "03 February",
                "04 February",
                "05 February",
                "06 February",
                "07 February"
            ],
            "labels": {
              "show": false
            },
            "axisBorder": {
              "show": false
            },
            "axisTicks": {
              "show": false
            }
          },
          "yaxis": {
                "show": false
            },
            "legend": {
                "show": false
            },
          "stroke": {
            "width": 6,
            "curve": "smooth"
          },
          "grid": {
            "show": false,
            "strokeDashArray": 4,
            "padding": {
              "left": 2,
              "right": 2,
              "top": 0
            }
          },
          "dataLabels": {
            "enabled": false
          },
          "tooltip": {
            "enabled": true,
            "x": {
              "show": false
            }
          }
        }"##;

        let id = el.get().unwrap().id();
        let chart = ApexChart::new(&id, ChartType::Area, series, raw_options)
            .expect("Could not create chart");

        chart.render();

        on_cleanup({
            let wrapped = SendWrapper::new(move || {
                chart.destroy();
            });
            move || wrapped()
        });
    });

    view! { <div id="plot" node_ref=el></div> }
}
```
