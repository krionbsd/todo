use druid::widget::Controller;
use druid::{Data, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, MouseButton, Widget};

pub struct DoubleClick<T> {
    /// A closure that will be invoked when the child widget is clicked.
    action: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
}

impl<T: Data> DoubleClick<T> {
    /// Create a new clickable [`Controller`] widget.
    pub fn new(action: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Self {
        Self {
            action: Box::new(action),
        }
    }
}

impl<T: Data, W: Widget<T>> Controller<T, W> for DoubleClick<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::MouseDown(m) = event  {
                if m.button == MouseButton::Left && m.count == 2 {
                        (self.action)(ctx, data, env);
                    }
                }
        child.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        env: &Env,
    ) {
        if let LifeCycle::HotChanged(_) | LifeCycle::FocusChanged(_) = event {
            ctx.request_paint();
        }

        child.lifecycle(ctx, event, data, env);
    }
}
