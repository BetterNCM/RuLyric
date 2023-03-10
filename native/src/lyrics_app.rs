use druid::widget::{Button, Flex, Label};
use druid::{Data, Env, EventCtx, Widget, WidgetExt};

use crate::model::font::FontConfig;
use crate::model::lyrics::LyricsData;
use crate::widgets::glow::Glow;
use crate::widgets::lyrics::LyricLineWidget;

#[derive(Data, Clone, Debug)]
pub struct LyricAppData {
    pub current_lyric: LyricsData,
    pub current_lyric_ext: LyricsData,
    #[data(eq)]
    pub win_data: Vec<LyricWinData>,
}

#[derive(Data, Clone, Debug, PartialEq)]
pub struct LyricWinData {
    pub with_words_lyrics: bool,
    pub font: FontConfig,
    pub font_secondary: FontConfig,
}

pub fn ui_builder(win_num: usize) -> impl Widget<LyricAppData> {
    let text = LyricLineWidget::new(move |data: &LyricAppData| {
        (
            data.current_lyric.clone(),
            data.win_data[win_num].font.clone(),
        )
    });

    let text2 = LyricLineWidget::new(move |data: &LyricAppData| {
        (
            data.current_lyric_ext.clone(),
            data.win_data[win_num].font_secondary.clone(),
        )
    });

    Glow::new(
        Flex::column()
            .with_child(text)
            .with_child(text2)
            .align_vertical(druid::UnitPoint::CENTER)
            ,
        win_num,
    )
}
