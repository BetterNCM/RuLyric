use druid::Data;

use super::font::FontConfig;

#[derive(Clone, Data, Debug, PartialEq)]
pub struct LyricsWord {
    pub lyric_word: String,
    pub lyric_duration: u64,
}

#[derive(Clone, Debug, PartialEq,Data)]
pub struct LyricsData {
    pub lyric_str: String,
    pub lyric_line_num: usize,
    #[data(eq)]
    pub lyrics: Vec<LyricsWord>,
    pub with_words_lyrics: bool,
    pub paused: bool,
    pub start_time: u64,
    pub font: FontConfig,
}

static mut COUNT_LINE_NUM: usize = 0;

impl LyricsData {
    pub fn new_test(str: String) -> LyricsData {
        LyricsData::from_lyrics(
            str.split('\u{a0}')
                .map(|s| LyricsWord {
                    lyric_word: s.to_string(),
                    lyric_duration: 200,
                })
                .collect(),unsafe {
                    COUNT_LINE_NUM += 1;
                    COUNT_LINE_NUM
                }
        )
    }
    pub fn from_lyrics(lyrics: Vec<LyricsWord>, line_num: usize) -> LyricsData {
        LyricsData {
            font: FontConfig {
                font_family: "Noto Sans SC".to_string(),
                font_size: 20.,
                font_color: druid::Color::WHITE,
            },
            lyric_line_num: line_num,
            lyric_str: lyrics
                .iter()
                .map(|word| word.lyric_word.clone())
                .collect::<Vec<String>>()
                .join(""),
            lyrics,
            with_words_lyrics: true,
            paused: false,
            start_time: 0,
        }
    }
}

impl LyricsData {
    pub fn get_per_word_lyrics_time(&self, current_time: u64) -> (usize, f64) {
        let mut time = 0;
        let mut words = 0;

        for t in &self.lyrics {
            if time + t.lyric_duration > current_time {
                return (
                    words,
                    (current_time - time) as f64 / t.lyric_duration as f64,
                );
            }
            time += t.lyric_duration;
            words += 1;
        }
        (words, 0.)
    }
}
