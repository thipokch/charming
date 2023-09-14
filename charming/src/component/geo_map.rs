use serde::{Deserialize, Serialize};
use macros::serde_auto;

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GeoMapOpt {
    #[serde(rename = "geoJSON")]
    GeoJson {
        value: serde_json::Value,
        special_areas: serde_json::Value,
    },
    #[serde(rename = "svg")]
    Svg(String),
}

impl<S> From<S> for GeoMapOpt
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        GeoMapOpt::Svg(s.into())
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoMap {
    name: Option<String>,

    opt: Option<GeoMapOpt>,
}

impl GeoMap {
    pub fn new() -> Self {
        GeoMap {
            name: None,
            opt: None,
        }
    }

    pub fn map_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn opt<M: Into<GeoMapOpt>>(mut self, opt: M) -> Self {
        self.opt = Some(opt.into());
        self
    }
}

impl From<&str> for GeoMap {
    fn from(svg: &str) -> Self {
        GeoMap::new().opt(GeoMapOpt::Svg(svg.to_string()))
    }
}

impl From<(&str, &str)> for GeoMap {
    fn from((name, svg): (&str, &str)) -> Self {
        GeoMap::new()
            .map_name(name.to_string())
            .opt(GeoMapOpt::Svg(svg.to_string()))
    }
}
