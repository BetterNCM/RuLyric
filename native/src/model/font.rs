use druid::{Color, Data, FontWeight, FontFamily};

#[derive(Clone, Debug, PartialEq,Data)]
pub struct FontConfig{
    pub font_family: String,
    pub font_size: f64,
    pub font_color:Color,
    pub font_weight: FontWeight
}