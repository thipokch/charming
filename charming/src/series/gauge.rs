use serde::{Deserialize, Serialize};
use macros::serde_auto;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        Anchor, AxisLabel, AxisLine, AxisTick, Color, ColorBy, Formatter, ItemStyle, Pointer,
        SplitLine,
    },
};

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GaugeDetail {
    show: Option<bool>,

    color: Option<Color>,

    font_style: Option<String>,

    font_weight: Option<String>,

    font_family: Option<String>,

    font_size: Option<f64>,

    precision: Option<f64>,

    value_animation: Option<bool>,

    formatter: Option<Formatter>,
}

impl GaugeDetail {
    pub fn new() -> Self {
        Self {
            show: None,
            color: None,
            font_style: None,
            font_weight: None,
            font_family: None,
            font_size: None,
            precision: None,
            value_animation: None,
            formatter: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn font_style<S: Into<String>>(mut self, font_style: S) -> Self {
        self.font_style = Some(font_style.into());
        self
    }

    pub fn font_weight<S: Into<String>>(mut self, font_weight: S) -> Self {
        self.font_weight = Some(font_weight.into());
        self
    }

    pub fn font_family<S: Into<String>>(mut self, font_family: S) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    pub fn font_size<F: Into<f64>>(mut self, font_size: F) -> Self {
        self.font_size = Some(font_size.into());
        self
    }

    pub fn precision<F: Into<f64>>(mut self, precision: F) -> Self {
        self.precision = Some(precision.into());
        self
    }

    pub fn value_animation(mut self, value_animation: bool) -> Self {
        self.value_animation = Some(value_animation);
        self
    }

    pub fn formatter<F: Into<Formatter>>(mut self, formatter: F) -> Self {
        self.formatter = Some(formatter.into());
        self
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GaugeTitle {
    show: Option<bool>,

    offset_center: Option<(String, String)>,
}

impl GaugeTitle {
    pub fn new() -> Self {
        Self {
            show: None,
            offset_center: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn offset_center<S: Into<String>>(mut self, offset_center: (S, S)) -> Self {
        self.offset_center = Some((offset_center.0.into(), offset_center.1.into()));
        self
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GaugeProgress {
    show: Option<bool>,

    overlap: Option<bool>,

    width: Option<f64>,

    round_cap: Option<bool>,

    clip: Option<bool>,

    item_style: Option<ItemStyle>,
}

impl GaugeProgress {
    pub fn new() -> Self {
        Self {
            show: None,
            overlap: None,
            width: None,
            round_cap: None,
            clip: None,
            item_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn overlap(mut self, overlap: bool) -> Self {
        self.overlap = Some(overlap);
        self
    }

    pub fn width<F: Into<f64>>(mut self, width: F) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn round_cap(mut self, round_cap: bool) -> Self {
        self.round_cap = Some(round_cap);
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }

    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }
}

#[serde_auto]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gauge {
    id: Option<String>,

    name: Option<String>,

    color_by: Option<ColorBy>,

    zlevel: Option<f64>,

    z: Option<f64>,

    center: Option<(String, String)>,

    legend_hover_link: Option<bool>,

    start_angle: Option<f64>,

    end_angle: Option<f64>,

    clockwise: Option<bool>,

    min: Option<f64>,

    max: Option<f64>,

    split_number: Option<f64>,

    radius: Option<String>,

    progress: Option<GaugeProgress>,

    axis_line: Option<AxisLine>,

    axis_tick: Option<AxisTick>,

    axis_label: Option<AxisLabel>,

    split_line: Option<SplitLine>,

    pointer: Option<Pointer>,

    anchor: Option<Anchor>,

    detail: Option<GaugeDetail>,

    title: Option<GaugeTitle>,


    data: DataFrame,
}

impl Gauge {
    pub fn new() -> Self {
        Self {
                        // type_: "gauge".to_string(),
            id: None,
            name: None,
            color_by: None,
            zlevel: None,
            z: None,
            center: None,
            legend_hover_link: None,
            start_angle: None,
            end_angle: None,
            clockwise: None,
            min: None,
            max: None,
            split_number: None,
            radius: None,
            progress: None,
            axis_line: None,
            axis_tick: None,
            axis_label: None,
            split_line: None,
            pointer: None,
            anchor: None,
            detail: None,
            title: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color_by<C: Into<ColorBy>>(mut self, color_by: C) -> Self {
        self.color_by = Some(color_by.into());
        self
    }

    pub fn zlevel<F: Into<f64>>(mut self, zlevel: F) -> Self {
        self.zlevel = Some(zlevel.into());
        self
    }

    pub fn z<F: Into<f64>>(mut self, z: F) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn center<S: Into<String>>(mut self, center: (S, S)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn start_angle<F: Into<f64>>(mut self, start_angle: F) -> Self {
        self.start_angle = Some(start_angle.into());
        self
    }

    pub fn end_angle<F: Into<f64>>(mut self, end_angle: F) -> Self {
        self.end_angle = Some(end_angle.into());
        self
    }

    pub fn clockwise(mut self, clockwise: bool) -> Self {
        self.clockwise = Some(clockwise);
        self
    }

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn split_number<F: Into<f64>>(mut self, split_number: F) -> Self {
        self.split_number = Some(split_number.into());
        self
    }

    pub fn radius<S: Into<String>>(mut self, radius: S) -> Self {
        self.radius = Some(radius.into());
        self
    }

    pub fn progress<P: Into<GaugeProgress>>(mut self, progress: P) -> Self {
        self.progress = Some(progress.into());
        self
    }

    pub fn axis_line<L: Into<AxisLine>>(mut self, axis_line: L) -> Self {
        self.axis_line = Some(axis_line.into());
        self
    }

    pub fn axis_tick<T: Into<AxisTick>>(mut self, axis_tick: T) -> Self {
        self.axis_tick = Some(axis_tick.into());
        self
    }

    pub fn axis_label<L: Into<AxisLabel>>(mut self, axis_label: L) -> Self {
        self.axis_label = Some(axis_label.into());
        self
    }

    pub fn split_line<L: Into<SplitLine>>(mut self, split_line: L) -> Self {
        self.split_line = Some(split_line.into());
        self
    }

    pub fn pointer<P: Into<Pointer>>(mut self, pointer: P) -> Self {
        self.pointer = Some(pointer.into());
        self
    }

    pub fn anchor<A: Into<Anchor>>(mut self, anchor: A) -> Self {
        self.anchor = Some(anchor.into());
        self
    }

    pub fn detail<D: Into<GaugeDetail>>(mut self, detail: D) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn title<T: Into<GaugeTitle>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
