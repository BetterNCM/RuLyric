use druid::widget::{Flex};
use druid::{Widget, Data};

use crate::model::lyrics::LyricsData;
use crate::widgets::lyrics::LyricLineWidget;

#[derive(Data,Clone,Debug)]
pub struct LyricAppData{
    pub current_lyric: String
}

pub fn ui_builder() -> impl Widget<LyricAppData> {
    let text = LyricLineWidget::new(|data:&LyricAppData|{
        LyricsData::new(&data.current_lyric)
    });

    Flex::column().with_child(text)
}