use druid::Data;

#[derive(Clone,Data,Debug,PartialEq)]
pub struct LyricsData {
    pub lyric_str: String,
}

impl LyricsData{
    pub fn new(str:&String) -> LyricsData{
        LyricsData{
            lyric_str:str.to_string()
        }
    }
}