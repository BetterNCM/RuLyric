use std::marker::PhantomData;

use druid::{
    piet::{D2DTextLayout, Text, TextAttribute, TextLayout, TextLayoutBuilder},
    Color, Data, Event, HasRawWindowHandle, Insets, LifeCycle, Point, RenderContext, Size, Widget,
};

use raw_window_handle_5::RawWindowHandle;

use crate::model::{font::FontConfig, lyrics::LyricsData};

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct LyricLineWidget<T, F: Fn(&T) -> (LyricsData, FontConfig)> {
    a: PhantomData<T>,
    lyric_line: Option<LyricsData>,
    lyric_line_updater: F,
    lyric_text_bg: Option<D2DTextLayout>,
    current_time: u64,
    space_width: f64,
    x_movement: f64,
    font_data: Option<FontConfig>,
}

impl<T, F: Fn(&T) -> (LyricsData, FontConfig)> LyricLineWidget<T, F> {
    pub fn new(updater: F) -> LyricLineWidget<T, F> {
        LyricLineWidget {
            lyric_line: None,
            lyric_line_updater: updater,
            a: PhantomData,
            lyric_text_bg: None,
            current_time: 0,
            space_width: 0.,
            x_movement: 0.,
            font_data: None,
        }
    }
}

impl<T: Data, F: Fn(&T) -> (LyricsData, FontConfig)> Widget<T> for LyricLineWidget<T, F> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        _data: &mut T,
        _env: &druid::Env,
    ) {
        match event {
            Event::MouseDown(_e) => {}
            Event::MouseMove(_m) => {
                use winapi::um::winuser::*;
                unsafe {
                    if let RawWindowHandle::Win32(handle) = ctx.window().raw_window_handle() {
                        winapi::um::winuser::SetWindowPos(
                            handle.hwnd as _,
                            HWND_TOPMOST,
                            0,
                            0,
                            0,
                            0,
                            SWP_NOMOVE | SWP_NOSIZE,
                        );
                    }
                }

                
                
            }
            Event::AnimFrame(delta_t) => {
                if let Some(lyric_line) = &self.lyric_line {
                    if !lyric_line.paused {
                        self.current_time += delta_t / 1000 / 1000;
                        ctx.request_paint();
                        ctx.request_anim_frame();
                    }
                }
            }
            _ => (),
        }

        // if let Event::WindowConnected = event {}
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        _data: &T,
        _env: &druid::Env,
    ) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &T, data: &T, _env: &druid::Env) {
        let (new_lyric, new_font) = (self.lyric_line_updater)(data);

        if let Some(LyricsData {
            start_time,
            lyric_line_num,
            ..
        }) = &self.lyric_line
        {
            // Update lyric time
            if *start_time != new_lyric.start_time || *lyric_line_num != new_lyric.lyric_line_num {
                self.current_time = new_lyric.start_time;
                self.x_movement = 0.;
            }
        } else {
            self.current_time = new_lyric.start_time;
            self.x_movement = 0.;
        }

        

        if self.lyric_line.is_none()
            || self.lyric_line.as_ref().unwrap().lyric_line_num != new_lyric.lyric_line_num
        {
            let t = ctx.text();

            let font = &new_font;
            self.lyric_text_bg = Some(
                t.new_text_layout(new_lyric.lyric_str.clone())
                    .font(
                        t.font_family(font.font_family.as_str())
                            .unwrap_or(druid::FontFamily::SYSTEM_UI),
                        font.font_size,
                    )
                    .default_attribute(TextAttribute::Weight(font.font_weight))
                    .build()
                    .unwrap(),
            );

            self.space_width = {
                let t = ctx.text();

                let mut get_width = |char: &str| {
                    t.new_text_layout(char.to_string())
                        .font(
                            t.font_family(font.font_family.as_str())
                                .unwrap_or(druid::FontFamily::SYSTEM_UI),
                            font.font_size,
                        )
                        .default_attribute(TextAttribute::Weight(font.font_weight))
                        .build()
                        .unwrap()
                        .size()
                        .width
                };

                get_width("_ _") - 2. * get_width("_")
            };

            if !new_lyric.paused {
                ctx.request_anim_frame();
            }

            self.lyric_line = Some(new_lyric);

            ctx.request_paint();
            ctx.request_layout();
        }

        self.font_data = Some(new_font);
    }
    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &T,
        _env: &druid::Env,
    ) -> druid::Size {
        if let Some(text) = &self.lyric_text_bg {
            let mut size = text.size();
            let winw = ctx.window().get_size().width;
            if winw < size.width {
                size.width = winw;
            }
            size
        } else {
            Size::new(0., 0.)
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &T, _env: &druid::Env) {
        if let (Some(ref _text_bg), Some(ref lyric_line), Some(ref font)) =
            (&self.lyric_text_bg, &self.lyric_line, &self.font_data)
        {
            let lyrics_origin = Point::new(0. - self.x_movement, 0.);

            // ctx.draw_text(text_bg, lyrics_origin);

            let mut draw_word = |word: String,
                                 cur_x: &mut f64,
                                 complete: f64,
                                 ctx: &mut druid::PaintCtx,
                                 color: Color| {
                let t = ctx.text();

                let space_x = if word.ends_with(' ') {
                    self.space_width
                } else {
                    0.
                };

                let layout = t
                    .new_text_layout(word)
                    .text_color(color)
                    .font(
                        t.font_family(font.font_family.as_str())
                            .unwrap_or(druid::FontFamily::SYSTEM_UI),
                        font.font_size,
                    )
                    .default_attribute(TextAttribute::Weight(font.font_weight))
                    .build()
                    .unwrap();

                let size = layout.size();
                let pos = Point::new(lyrics_origin.x + *cur_x, lyrics_origin.y);

                if complete != 1. {
                    let cur_width = size.width * complete + pos.x + self.x_movement;

                    macro_rules! min {
                            ($x: expr) => ($x);
                            ($x: expr, $($z: expr),+) => {{
                                let y = min!($($z),*);
                                if $x < y {
                                    $x
                                } else {
                                    y
                                }
                            }}
                        }

                    macro_rules! max {
                            ($x: expr) => ($x);
                            ($x: expr, $($z: expr),+) => {{
                                let y = max!($($z),*);
                                if $x > y {
                                    $x
                                } else {
                                    y
                                }
                            }}
                        }

                    let winw = ctx.window().get_size().width;
                    self.x_movement = {
                        let mv = cur_width + winw / 2. - winw;
                        let maxmv = self.lyric_text_bg.as_ref().unwrap().size().width - winw;
                        max!(min!(mv, maxmv), 0.)
                    };

                    ctx.save().unwrap();
                    ctx.clip(druid::kurbo::Rect::from_origin_size(
                        pos,
                        (size.width * complete, size.height),
                    ));

                    ctx.draw_text(&layout, pos);
                    ctx.restore().unwrap();
                } else {
                    ctx.draw_text(&layout, pos);
                }

                *cur_x += size.width + space_x;
            };

            let mut cur_x = 0.;
            for (_index, word) in lyric_line.lyrics.iter().enumerate() {
                draw_word(
                    word.lyric_word.clone(),
                    &mut cur_x,
                    1.,
                    ctx,
                    font.font_background_color,
                );
            }

            let current_lyric = lyric_line.get_per_word_lyrics_time(self.current_time);
            let mut cur_x = 0.;

            for (index, word) in lyric_line.lyrics.iter().enumerate() {
                if index > current_lyric.0 {
                    break;
                }

                draw_word(
                    word.lyric_word.clone(),
                    &mut cur_x,
                    if index < current_lyric.0 {
                        1.
                    } else {
                        current_lyric.1
                    },
                    ctx,
                    font.font_color,
                );
            }
        }
    }
}
