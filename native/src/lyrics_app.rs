use druid::widget::{Flex};
use druid::{Widget, Data};

use crate::model::lyrics::LyricsData;
use crate::widgets::lyrics::LyricLineWidget;

#[derive(Data,Clone,Debug)]
pub struct LyricAppData{
    pub current_lyric: LyricsData
}

pub fn ui_builder() -> impl Widget<LyricAppData> {
    let text = LyricLineWidget::new(|data:&LyricAppData|{
        data.current_lyric.clone()
    });

    Flex::column().with_child(text)
}