use std::{marker::PhantomData, sync::Arc};

use druid::{
    piet::{Text, TextLayoutBuilder},
    widget::{Label, RawLabel},
    ArcStr, Data, Event, Insets, LifeCycle, Point, RenderContext, Size, Widget, HasRawWindowHandle,
};


use raw_window_handle_5::RawWindowHandle;
use winapi::um::winuser::{HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};

use crate::model::lyrics::LyricsData;

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct LyricLine<T, F: Fn(&T) -> LyricsData> {
    a: PhantomData<T>,
    lyric_line: Option<LyricsData>,
    lyric_line_updater: F,
    label: RawLabel<ArcStr>,
}

impl<T, F: Fn(&T) -> LyricsData> LyricLine<T, F> {
    pub fn new(updater: F) -> LyricLine<T, F> {
        LyricLine {
            lyric_line: None,
            label: RawLabel::new(),
            lyric_line_updater: updater,
            a: PhantomData,
        }
    }
}

impl<T: Data, F: Fn(&T) -> LyricsData> Widget<T> for LyricLine<T, F> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) {
        if let Event::MouseMove(m) = event {
            ctx.window().handle_titlebar(true);
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

        if let Event::WindowConnected = event {
            
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {
        let lyric = Some((self.lyric_line_updater)(data));
        if self.lyric_line != lyric {
            self.lyric_line = lyric;
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        Size::new(400., 400.)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        let t = ctx.text();

        let s = self
            .lyric_line
            .clone()
            .unwrap_or(LyricsData::new(&"".to_string()));

        let text = t
            .new_text_layout(s.lyric_str)
            .text_color(druid::Color::WHITE)
            .font(t.font_family("Noto Sans SC").unwrap(), 20.)
            .build()
            .unwrap();

        ctx.draw_text(&text, Point::new(10., 5.));
    }
}
