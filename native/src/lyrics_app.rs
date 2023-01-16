use druid::widget::{Flex};
use druid::{Widget, Data};

use crate::model::lyrics::LyricsData;
use crate::widgets::lyrics::LyricLineWidget;

#[derive(Data,Clone,Debug)]
pub struct LyricAppData{
    pub current_lyric: LyricsData,
    pub current_lyric_ext: LyricsData
}

pub fn ui_builder() -> impl Widget<LyricAppData> {
    let text = LyricLineWidget::new(|data:&LyricAppData|{
        data.current_lyric.clone()
    });

    let text2 = LyricLineWidget::new(|data:&LyricAppData|{
        data.current_lyric_ext.clone()
    });

    Flex::column().with_child(text).with_child(text2)
}