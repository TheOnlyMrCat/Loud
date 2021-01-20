use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget};
use druid::widget::{List, Split};

use crate::project::Project;

#[derive(Clone, Data)]
pub struct ProjectState {
	project: Project,
}

pub struct TimelineView;

impl<T> Widget<T> for TimelineView {
	fn event(&mut self, ctx: &mut EventCtx<'_, '_>, event: &Event, data: &mut T, env: &Env) {

	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx<'_, '_>, event: &LifeCycle, data: &T, env: &Env) {

	}

	fn update(&mut self, ctx: &mut UpdateCtx<'_, '_>, old_data: &T, data: &T, env: &Env) {

	}

	fn layout(&mut self, ctx: &mut LayoutCtx<'_, '_>, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		bc.constrain((bc.max().width, 20.0))
	}

	fn paint(&mut self, ctx: &mut PaintCtx<'_, '_, '_>, data: &T, env: &Env) {

	}
}

pub struct Instrument;

pub fn build_root() -> impl Widget<ProjectState> {
	Split::rows(
		TimelineView,

	)
}