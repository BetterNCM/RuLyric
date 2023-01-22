use druid::widget::prelude::*;
use druid::widget::{
    Align, BackgroundBrush, Button, Controller, ControllerHost, Flex, Label, Padding,
};
use druid::Target::Global;
use druid::{
    commands as sys_cmds, AppDelegate, AppLauncher, Application, Color, Command, Data, DelegateCtx,
    Handled, HasRawWindowHandle, LocalizedString, Menu, MenuItem, RawWindowHandle, Target,
    WindowDesc, WindowHandle, WindowId,
};
use winapi::um::winuser::{HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};

use crate::lyrics_app::LyricAppData;

pub struct Glow<W> {
    inner: W,
    winid: usize,
}

impl<W> Glow<W> {
    pub fn new(inner: W, winid: usize) -> Glow<W> {
        Glow { inner, winid }
    }
}

impl<W: Widget<LyricAppData>> Widget<LyricAppData> for Glow<W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut LyricAppData, env: &Env) {
        if let Event::MouseMove(_) = event {
            ctx.window().handle_titlebar(true);
            // unsafe {
            //     if let RawWindowHandle::Win32(handle) = ctx.window().raw_window_handle() {
            //         winapi::um::winuser::SetWindowPos(
            //             handle.hwnd as _,
            //             HWND_TOPMOST,
            //             0,
            //             0,
            //             0,
            //             0,
            //             SWP_NOMOVE | SWP_NOSIZE,
            //         );
            //     }
            // }
        }

        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &LyricAppData,
        env: &Env,
    ) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &LyricAppData,
        data: &LyricAppData,
        env: &Env,
    ) {
        ctx.request_paint();
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &LyricAppData,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &LyricAppData, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}
