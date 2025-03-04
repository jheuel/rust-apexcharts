use indexmap::IndexMap;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys;

/// Represents the type of the chart that will be rendered.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum ChartType {
    /// Represents an area chart.
    Area,
    /// Represents a bar chart.
    Bar,
    /// Represents a line chart.
    Line,
    /// Represents a Boxplot chart.
    BoxPlot,
    /// Represents a candlestick chart.
    CandleStick,
    /// Represents a range bar chart.
    RangeBar,
    /// Represents a range area chart.
    RangeArea,
    /// Represents a heatmap chart.
    HeatMap,
    /// Represents a treemap chart.
    Treemap,
    /// Represents a funnel chart.
    Funnel,
    /// Represents a multi-axis chart.
    MultiAxis,
    /// Represents a pie chart. The expected type of data is [SeriesData::Radial].
    Pie,
    /// Represents a donut chart. The expected type of data is [SeriesData::Radial].
    Donut,
    /// Represents a radar chart.
    Radar,
    /// Represents a radial bar chart. The expected type of data is [SeriesData::Radial].
    RadialBar,
    /// Represents a circular gauge chart. The expected type of data is [SeriesData::Radial].
    CircularGauge,
}

impl Display for ChartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartType::Area => write!(f, "area"),
            ChartType::Bar => write!(f, "bar"),
            ChartType::Line => write!(f, "line"),
            ChartType::BoxPlot => write!(f, "boxPlot"),
            ChartType::CandleStick => write!(f, "candlestick"),
            ChartType::RangeBar => write!(f, "rangeBar"),
            ChartType::RangeArea => write!(f, "rangeArea"),
            ChartType::HeatMap => write!(f, "heatmap"),
            ChartType::Treemap => write!(f, "treemap"),
            ChartType::Funnel => write!(f, "funnel"),
            ChartType::MultiAxis => write!(f, "multiAxis"),
            ChartType::Pie => write!(f, "pie"),
            ChartType::Donut => write!(f, "donut"),
            ChartType::Radar => write!(f, "radar"),
            ChartType::RadialBar => write!(f, "radialBar"),
            ChartType::CircularGauge => write!(f, "radialBar"),
        }
    }
}

impl Serialize for ChartType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Represents the data that will be rendered in the chart.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum SeriesData {
    /// Represents a single array of data points. eg `[10, 20, 30]`
    Single(Vec<i64>),
    /// Represents a double array of data points. eg `[(10, 20), (20, 30)]`
    NumericPaired(Vec<(i64, i64)>),
    /// Represents a double array of data points with a category. eg `[("Apple", 30), ("Banana", 40)]`
    CategoryPaired(Vec<(String, i64)>),
    /// Represents a double array of data points with a timestamp. eg `[(1619683200, 30), (1619769600, 40)]`
    Timestamped(Vec<(i64, i64)>),
    /// Represents a double array of data points with a date. eg `[("2021-04-29", 30), ("2021-04-30", 40)]`
    Dated(Vec<(String, i64)>),
    /// Represents a double array of data points which is a percentage adding up to 100. eg `[("Apple", 30.0), ("Banana", 40.0), ("Orange", 30.0)]`. It is used primarily for the `Pie`, `Donut`, and `Radial` chart types.
    Radial(Vec<(String, f64)>),
    /// Represents a double array of data points for a candlestick chart. eg `[("Sun", [10.0, 20.0, 5.0, 15.0]), ("Mon", [15.0, 25.0, 10.0, 20.0])]`. The data points are in the order of `[Open, High, Low, Close]`.
    CandleStick(Vec<(String, Vec<f64>)>),
}

impl Serialize for SeriesData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SeriesData::Single(data) => {
                let mut seq = serializer.serialize_seq(Some(data.len()))?;
                for item in data {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            SeriesData::NumericPaired(data) | SeriesData::Timestamped(data) => {
                let mut seq = serializer.serialize_seq(Some(data.len()))?;
                let data = data.iter().map(|(x, y)| vec![*x, *y]).collect::<Vec<_>>();
                for item in data {
                    seq.serialize_element(&item)?;
                }
                seq.end()
            }
            SeriesData::CategoryPaired(data) | SeriesData::Dated(data) => {
                // Serialize the data into a sequence of an object with two properties `x` and `y`. eg `[{x: "Apple", y: 30}, {x: "Banana", y: 40}]`
                let mut seq = serializer.serialize_seq(Some(data.len()))?;
                let data: Vec<IndexMap<String, Value>> = data
                    .iter()
                    .map(|(x, y)| {
                        IndexMap::from_iter(vec![
                            ("x".to_string(), Value::String(x.to_string())),
                            ("y".to_string(), Value::Number(serde_json::Number::from(*y))),
                        ])
                    })
                    .collect::<Vec<_>>();
                for item in data {
                    seq.serialize_element(&item)?;
                }
                seq.end()
            }
            SeriesData::Radial(data) => {
                let mut seq = serializer.serialize_seq(Some(data.len()))?;
                let data: Vec<IndexMap<String, Value>> = data
                    .iter()
                    .map(|(x, y)| {
                        IndexMap::from_iter(vec![
                            ("x".to_string(), Value::String(x.to_string())),
                            (
                                "y".to_string(),
                                Value::Number(serde_json::Number::from_f64(*y).unwrap()),
                            ),
                        ])
                    })
                    .collect::<Vec<_>>();
                for item in data {
                    seq.serialize_element(&item)?;
                }
                seq.end()
            }
            SeriesData::CandleStick(data) => {
                let mut seq = serializer.serialize_seq(Some(data.len()))?;
                let data: Vec<IndexMap<String, Value>> = data
                    .iter()
                    .map(|(x, y)| {
                        IndexMap::from_iter(vec![
                            ("x".to_string(), Value::String(x.to_string())),
                            (
                                "y".to_string(),
                                Value::Array(
                                    y.iter()
                                        .map(|v| {
                                            Value::Number(serde_json::Number::from_f64(*v).unwrap())
                                        })
                                        .collect::<Vec<_>>(),
                                ),
                            ),
                        ])
                    })
                    .collect::<Vec<_>>();
                for item in data {
                    seq.serialize_element(&item)?;
                }
                seq.end()
            }
        }
    }
}

/// Represents a series in the chart.
///
/// This type is used to represent a series in the chart. It contains the name of the series, the data
/// that will be rendered, the color of the series, the type of the series, and the z-index of the series.
/// It is mostly used when you want to update the series in the chart dynamically.
///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartSeries {
    /// The name of the series. This is used to identify the series in the chart. It is also displayed in the legend.
    pub name: String,
    /// The data that will be rendered in the chart. Different types of charts require different types of data.
    pub data: SeriesData,
    /// The color of the series. This is used to set the color of the series in the chart.
    pub color: String,
    /// The type of the series. This is used to set the type of the series in the chart. Note that this
    /// overrides the type of the chart provided in the `ApexChartComponent` component. Usually, you don't need to set this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_type: Option<ChartType>,
    /// The z-index of the series. This is used to set the z-index of the series in the chart. It is used to determine
    /// the order in which the series are rendered in the chart. The series with the highest z-index is rendered on top.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i32>,
}

impl From<ChartSeries> for JsValue {
    fn from(chart_series: ChartSeries) -> JsValue {
        let series = js_sys::Object::new();
        js_sys::Reflect::set(
            &series,
            &JsValue::from_str("name"),
            &JsValue::from_str(&chart_series.name),
        )
        .unwrap();
        js_sys::Reflect::set(
            &series,
            &JsValue::from_str("data"),
            &serde_wasm_bindgen::to_value(&chart_series.data).unwrap(),
        )
        .unwrap();
        js_sys::Reflect::set(
            &series,
            &JsValue::from_str("color"),
            &JsValue::from_str(&chart_series.color),
        )
        .unwrap();

        if let Some(series_type) = chart_series.chart_type {
            js_sys::Reflect::set(
                &series,
                &JsValue::from_str("type"),
                &JsValue::from_str(&series_type.to_string()),
            )
            .unwrap();
        }
        js_sys::Reflect::set(
            &series,
            &JsValue::from_str("zIndex"),
            &JsValue::from_f64(chart_series.z_index.unwrap_or(0) as f64),
        )
        .unwrap();
        series.into()
    }
}

/// A helper function to convert a vector of items into a JsValue.
pub fn to_jsvalue<T: Into<JsValue>>(vec: Vec<T>) -> JsValue {
    let array = js_sys::Array::new();
    for item in vec {
        array.push(&item.into());
    }
    array.into()
}

#[cfg(test)]
mod tests {
    use crate::prelude::SeriesData;

    #[test]
    pub fn test_series_data_serialization() {
        let single_data = serde_json::to_string(&SeriesData::Single(vec![10, 20, 30])).unwrap();
        assert_eq!(single_data, "[10,20,30]");

        let numeric_paired_data =
            serde_json::to_string(&SeriesData::NumericPaired(vec![(10, 20), (20, 30)])).unwrap();
        assert_eq!(numeric_paired_data, "[[10,20],[20,30]]");

        let category_paired_data = serde_json::to_string(&SeriesData::CategoryPaired(vec![
            ("Apple".to_string(), 30),
            ("Banana".to_string(), 40),
        ]))
        .unwrap();
        assert_eq!(
            category_paired_data,
            r#"[{"x":"Apple","y":30},{"x":"Banana","y":40}]"#
        );

        let timestamped_data = serde_json::to_string(&SeriesData::Timestamped(vec![
            (1619683200, 30),
            (1619769600, 40),
        ]))
        .unwrap();
        assert_eq!(timestamped_data, "[[1619683200,30],[1619769600,40]]");

        let dated_data = serde_json::to_string(&SeriesData::Dated(vec![
            ("2021-04-29".to_string(), 30),
            ("2021-04-30".to_string(), 40),
        ]))
        .unwrap();
        assert_eq!(
            dated_data,
            r#"[{"x":"2021-04-29","y":30},{"x":"2021-04-30","y":40}]"#
        );
    }
}
