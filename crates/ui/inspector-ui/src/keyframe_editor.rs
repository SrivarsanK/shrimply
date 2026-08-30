use shrimply_ui_foundation::tr;
use shrimply_ui_foundation::ui::I18nWidgetExt;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use shrimply_core::timeline_value::TextInterpolation;
use shrimply_interpolation::Interpolation;
use shrimply_state::preferences;
use uuid::Uuid;

use crate::Color;
use crate::player_state;
use crate::timeline::renderer::{
    Rect, Stroke, StrokeKind, TimelinePainter, TimelineRenderer, Vec2, vec2,
};
use shrimply_project::project::{ItemAddress, Project, Time};

pub(crate) use super::keyframe_graph::{KeyframeGraph, KeyframePoint, RawSegment, SpeedSegment};
use super::{InspectorContext, keyframe_graph::GraphDomain, keyframe_model};

const GRAPH_CONTENT_HEIGHT: i32 = 112;
const GRAPH_HEIGHT: i32 = GRAPH_CONTENT_HEIGHT + GRAPH_SLIDER_HEIGHT as i32;
pub(super) const GRAPH_PAD: f64 = 12.0;
pub(super) const STEP_GRAPH_RANGE: (f64, f64) = (-0.15, 1.15);
pub(super) const CURSOR_LANE_HEIGHT: f64 = 18.0;
pub(super) const GRAPH_SLIDER_HEIGHT: f64 = 20.0;
const GRAPH_SCROLLBAR_WHEEL_PAGE_FRACTION: f64 = 0.25;
const HIT_RADIUS: f64 = 7.0;
pub(super) const SPEED_CURVE_STEPS: usize = 48;
const CURVE_BREAK_OFFSET: f64 = 1.0 / (SPEED_CURVE_STEPS as f64 * 64.0);
const KEYFRAME_CLIPBOARD_MARKER: &str = "shrimply keyframes";

thread_local! {
    static KEYFRAME_CLIPBOARD: RefCell<Option<keyframe_model::KeyframeClipboard>> = const { RefCell::new(None) };
}

pub(crate) struct BuiltKeyframeEditor {
    pub(crate) widget: gtk::Widget,
    pub(crate) update_graph: Rc<dyn Fn(KeyframeGraph)>,
}

pub(crate) type CopyKeyframes = Rc<dyn Fn(&[Time]) -> Option<keyframe_model::KeyframeClipboard>>;
pub(crate) type PasteKeyframes =
    Rc<dyn Fn(&keyframe_model::KeyframeClipboard, &[Time]) -> Option<Vec<Time>>>;

pub(crate) struct KeyframeEditorActions {
    pub(crate) add_at_time: Rc<dyn Fn(Time)>,
    pub(crate) delete_at_time: Rc<dyn Fn(Time)>,
    pub(crate) update_point: Rc<dyn Fn(Time, Time, f64)>,
    pub(crate) copy_keyframes: CopyKeyframes,
    pub(crate) paste_keyframes: PasteKeyframes,
    pub(crate) set_interpolation: Option<Rc<dyn Fn(Uuid, Interpolation)>>,
    pub(crate) text_interpolation: Option<TextInterpolationActions>,
    pub(crate) toggle_playback: Rc<dyn Fn()>,
}

pub(crate) struct TextInterpolationActions {
    pub(crate) get: Rc<dyn Fn(Uuid) -> Option<TextInterpolation>>,
    pub(crate) set: Rc<dyn Fn(Uuid, TextInterpolation)>,
}

type TextInterpolationSelection = (TextInterpolation, Rc<dyn Fn(Uuid, TextInterpolation)>);

#[derive(Clone, Copy)]
enum DragTarget {
    Point(KeyframePoint),
    Cursor,
    SelectBox,
    SliderMove,
}

#[derive(Default)]
struct KeyframeSelection {
    focused: Option<Time>,
    selected: Vec<Time>,
}

#[derive(Clone, Copy)]
struct GraphSelectionBox {
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    add_to_selection: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct GraphViewState {
    scroll_seconds: f64,
    seconds_per_pixel: f64,
    minimum_seconds_per_pixel: Option<f64>,
    visible_area: GraphDomain,
    drag_start_x: f64,
    drag_start_scroll_seconds: f64,
    initialized: bool,
}

impl Default for GraphViewState {
    fn default() -> Self {
        Self {
            scroll_seconds: 0.0,
            seconds_per_pixel: 1.0 / 60.0,
            minimum_seconds_per_pixel: None,
            visible_area: (Time::ZERO, Time::ZERO),
            drag_start_x: 0.0,
            drag_start_scroll_seconds: 0.0,
            initialized: false,
        }
    }
}

impl GraphViewState {
    fn initialize(&mut self, range: GraphDomain, width: f64) {
        if self.initialized || width <= 0.0 {
            return;
        }
        let duration = graph_duration_seconds(range);
        let plot_width = graph_plot_width(width);
        self.scroll_seconds = range.0.as_secs_f64();
        self.seconds_per_pixel = (duration / plot_width).max(self.minimum_scale(duration));
        self.initialized = true;
    }

    fn clamp(&mut self, range: GraphDomain, width: f64) {
        let duration = graph_duration_seconds(range);
        let plot_width = graph_plot_width(width);
        let min_seconds_per_pixel = self.minimum_scale(duration);
        let max_seconds_per_pixel = (duration / plot_width).max(min_seconds_per_pixel);
        self.seconds_per_pixel = self
            .seconds_per_pixel
            .clamp(min_seconds_per_pixel, max_seconds_per_pixel);
        let visible_seconds = self.visible_seconds(width).clamp(0.0, duration);
        if visible_seconds >= duration || width <= 0.0 {
            self.scroll_seconds = range.0.as_secs_f64();
            self.seconds_per_pixel = (duration / plot_width).max(min_seconds_per_pixel);
            return;
        }
        let min_scroll = range.0.as_secs_f64();
        let max_scroll = range.1.as_secs_f64() - visible_seconds;
        self.scroll_seconds = self
            .scroll_seconds
            .clamp(min_scroll, max_scroll.max(min_scroll));
    }

    fn domain(&self, range: GraphDomain, width: f64) -> GraphDomain {
        let start = self.scroll_seconds;
        let end = (start + self.visible_seconds(width)).min(range.1.as_secs_f64());
        (
            Time::from_seconds_f64(start),
            Time::from_seconds_f64(end.max(start)),
        )
    }

    fn visible_seconds(&self, width: f64) -> f64 {
        graph_plot_width(width) * self.seconds_per_pixel
    }

    fn item_range(&self) -> GraphDomain {
        graph_domain(self.visible_area)
    }

    fn minimum_scale(&self, duration: f64) -> f64 {
        self.minimum_seconds_per_pixel
            .unwrap_or_else(|| min_graph_seconds_per_pixel(duration))
    }
}

pub(crate) type SharedGraphViewState = Rc<RefCell<GraphViewState>>;

pub(crate) fn new_graph_view_state() -> SharedGraphViewState {
    Rc::new(RefCell::new(GraphViewState::default()))
}

#[derive(Clone, Copy)]
struct GraphOverscroll {
    edge: GraphOverscrollEdge,
    started_at: Instant,
    distance: f64,
}

use shrimply_skia_adw_ui::Edge as GraphOverscrollEdge;

pub(crate) fn project_frame_step(project: &Project) -> Time {
    project.frame_step()
}

pub(crate) fn project_frame_keyframe_time(
    project: &Project,
    item: Option<&ItemAddress>,
    time: Time,
) -> Option<Time> {
    let Some(item) = item else {
        return Some(time.snapped(project.frame_step()));
    };
    project
        .keyframe_timeline_time(item, time)
        .and_then(|timeline_time| project.keyframe_time(item, timeline_time))
}

pub(crate) fn build(
    context: &InspectorContext,
    value_editor: gtk::Widget,
    graph: KeyframeGraph,
    visible_area: (Time, Time),
    view_state_scope: impl Into<String>,
    actions: KeyframeEditorActions,
) -> BuiltKeyframeEditor {
    let frame_step = {
        let project = context.project.borrow();
        context
            .selected_item
            .as_ref()
            .and_then(|item| project.keyframe_step(item))
            .filter(|step| *step > Time::ZERO)
            .unwrap_or_else(|| project.frame_step())
    };
    let graph_view = context.keyframe_graph_view_state(view_state_scope);
    let project = context.project.clone();
    let selected_item = context.selected_item.clone();
    let visible_area =
        clip_bounded_visible_area(&project.borrow(), selected_item.as_ref(), visible_area);
    {
        let mut graph_view = graph_view.borrow_mut();
        graph_view.minimum_seconds_per_pixel =
            matches!(graph, KeyframeGraph::Step { .. }).then(|| {
                frame_step.as_secs_f64() / shrimply_discrete_keyframe_graph_ui::MAX_FRAME_WIDTH
            });
        graph_view.visible_area = visible_area;
    }
    let preferences = context.preferences.clone();
    let playhead = {
        let project = context.project.clone();
        let player_state = context.player_state.clone();
        let selected_item = context.selected_item.clone();
        Rc::new(move || {
            let position = player_state::snapshot(&player_state).position;
            selected_item
                .as_ref()
                .and_then(|key| project.borrow().keyframe_time(key, position))
                .unwrap_or(position)
        })
    };
    let select_time = {
        let project = context.project.clone();
        let player_state = context.player_state.clone();
        let selected_item = context.selected_item.clone();
        Rc::new(move |time| {
            let position = selected_item
                .as_ref()
                .and_then(|key| project.borrow().keyframe_timeline_time(key, time))
                .unwrap_or(time);
            player_state::seek_time(&player_state, position);
        })
    };
    let root = gtk::Box::new(gtk::Orientation::Vertical, 6);
    root.set_hexpand(true);

    let row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    row.set_hexpand(true);
    value_editor.set_hexpand(true);
    row.append(&value_editor);

    let previous = gtk::Button::from_icon_name("go-previous-symbolic");
    previous.set_tooltip_i18n("Previous keyframe");
    previous.add_css_class("flat");
    row.append(&previous);

    let add = gtk::Button::from_icon_name("list-add-symbolic");
    add.set_tooltip_i18n("Add keyframe at playhead");
    add.add_css_class("flat");
    row.append(&add);

    let next = gtk::Button::from_icon_name("go-next-symbolic");
    next.set_tooltip_i18n("Next keyframe");
    next.add_css_class("flat");
    row.append(&next);

    root.append(&row);

    let graph_height = if matches!(graph, KeyframeGraph::Step { .. }) {
        shrimply_discrete_keyframe_graph_ui::CONTENT_HEIGHT + GRAPH_SLIDER_HEIGHT as i32
    } else {
        GRAPH_HEIGHT
    };
    let graph_data = Rc::new(RefCell::new(graph));
    let pending_graph = Rc::new(RefCell::new(None));
    let pointer_pos = Rc::new(RefCell::new(None::<Vec2>));
    let selected_time = Rc::new(RefCell::new(None));
    let key_selection = Rc::new(RefCell::new(KeyframeSelection::default()));
    let selection_box = Rc::new(RefCell::new(None::<GraphSelectionBox>));
    let active_target: Rc<RefCell<Option<DragTarget>>> = Rc::new(RefCell::new(None));
    let overscroll: Rc<RefCell<Option<GraphOverscroll>>> = Rc::new(RefCell::new(None));
    let scrollbar_lifecycle = Rc::new(RefCell::new(
        shrimply_skia_adw_ui::slider::Lifecycle::default(),
    ));
    let animation_tick_active = Rc::new(RefCell::new(false));
    let logical_playhead = playhead();
    sync_keyframe_controls(
        &previous,
        &add,
        &next,
        &graph_data.borrow(),
        logical_playhead,
        logical_playhead,
        frame_step,
    );
    let graph = gtk::GLArea::builder()
        .auto_render(false)
        .has_depth_buffer(false)
        .has_stencil_buffer(false)
        .height_request(graph_height)
        .hexpand(true)
        .focusable(true)
        .build();

    {
        let previous = previous.clone();
        let add = add.clone();
        let next = next.clone();
        let graph_data = graph_data.clone();
        let pending_graph = pending_graph.clone();
        let graph_view = graph_view.clone();
        let key_selection = key_selection.clone();
        let selection_box = selection_box.clone();
        let overscroll = overscroll.clone();
        let scrollbar_lifecycle = scrollbar_lifecycle.clone();
        let pointer_pos = pointer_pos.clone();
        let animation_tick_active = animation_tick_active.clone();
        let playhead = playhead.clone();
        let renderer = Rc::new(RefCell::new(TimelineRenderer::new()));
        let render_renderer = renderer.clone();
        graph.connect_render(move |area, _| {
            shrimply_support::crash::set_context("keyframe graph render begin");
            if let Some(error) = area.error() {
                tracing::error!("Keyframe graph GLArea error: {error}");
                return glib::Propagation::Stop;
            }
            shrimply_support::crash::set_context("keyframe graph make_current begin");
            area.make_current();
            if let Some(error) = area.error() {
                tracing::error!("Keyframe graph GLArea error after make_current: {error}");
                return glib::Propagation::Stop;
            }
            shrimply_support::crash::set_context("keyframe graph make_current end");

            let width = area.width().max(1);
            let height = area.height().max(1);
            let pixels_per_point = area.scale_factor().max(1) as f32;
            let accent_color: Color = adw::StyleManager::for_display(&area.display())
                .accent_color_rgba()
                .into();
            let screen_size_px = glam::UVec2::new(
                (width as f32 * pixels_per_point).round().max(1.0) as u32,
                (height as f32 * pixels_per_point).round().max(1.0) as u32,
            );
            let mut renderer = render_renderer.borrow_mut();
            shrimply_support::crash::set_context(format!(
                "keyframe graph begin_frame size={}x{} scale={}",
                screen_size_px.x, screen_size_px.y, pixels_per_point
            ));
            let painter = match renderer.begin_frame(
                screen_size_px,
                pixels_per_point,
                shrimply_skia_adw_ui::theme::current().view_bg,
            ) {
                Ok(painter) => painter,
                Err(error) => {
                    tracing::error!("Could not initialize keyframe graph renderer: {error}");
                    return glib::Propagation::Stop;
                }
            };
            shrimply_support::crash::set_context("keyframe graph begin_frame end");
            if let Some((updated, visible_area)) = pending_graph.borrow_mut().take() {
                *graph_data.borrow_mut() = updated;
                graph_view.borrow_mut().visible_area = visible_area;
            }
            let graph_data = graph_data.borrow();
            let item_range = graph_view.borrow().item_range();
            let graph_width = width.max(1) as f64;
            let content_height = graph_content_height(height.max(1) as f64);
            let mut graph_view = graph_view.borrow_mut();
            graph_view.initialize(item_range, graph_width);
            let scroll_animating =
                apply_graph_scroll_animation(&mut graph_view, item_range, &scrollbar_lifecycle);
            graph_view.clamp(item_range, graph_width);
            let domain = graph_view.domain(item_range, graph_width);
            let base_scrollbar =
                graph_scrollbar(*graph_view, item_range, graph_width, height.max(1) as f64);
            let scrollbar_frame = {
                let mut lifecycle = scrollbar_lifecycle.borrow_mut();
                lifecycle.frame(base_scrollbar, *pointer_pos.borrow())
            };
            let overscroll_frame = overscroll.borrow().and_then(|overscroll| {
                let distance = shrimply_skia_adw_ui::overshoot_distance(
                    overscroll.distance,
                    overscroll.started_at.elapsed(),
                );
                (distance > shrimply_skia_adw_ui::OVERSHOOT_VISIBLE_DISTANCE)
                    .then_some((overscroll.edge, distance))
            });
            if overscroll_frame.is_none() {
                overscroll.borrow_mut().take();
            }
            let logical_playhead = playhead();
            sync_keyframe_controls(
                &previous,
                &add,
                &next,
                &graph_data,
                logical_playhead,
                logical_playhead,
                frame_step,
            );
            let key_selection = key_selection.borrow();
            shrimply_support::crash::set_context(format!(
                "keyframe graph draw begin size={}x{} selected={} focused={:?}",
                width,
                height,
                key_selection.selected.len(),
                key_selection.focused
            ));
            super::keyframe_graph::draw_keyframes(super::keyframe_graph::KeyframeGraphDraw {
                painter: &painter,
                width: graph_width,
                height: height.max(1) as f64,
                content_height,
                graph: &graph_data,
                domain,
                frame_step,
                scrollbar: scrollbar_frame.scrollbar,
                overscroll: overscroll_frame,
                playhead: logical_playhead,
                virtual_playhead: None,
                selected_keys: &key_selection.selected,
                focused_key: key_selection.focused,
                accent_color,
            });
            shrimply_support::crash::set_context("keyframe graph draw end");
            if let Some(selection_box) = *selection_box.borrow() {
                shrimply_support::crash::set_context("keyframe graph draw selection box");
                draw_selection_box(&painter, selection_box, content_height);
            }
            shrimply_support::crash::set_context("keyframe graph end_frame begin");
            if let Err(error) = renderer.end_frame() {
                tracing::error!("Could not finalize keyframe graph renderer: {error}");
            }
            shrimply_support::crash::set_context("keyframe graph end_frame end");
            if scroll_animating || scrollbar_frame.animating {
                start_graph_animation_tick(
                    area,
                    overscroll.clone(),
                    scrollbar_lifecycle.clone(),
                    animation_tick_active.clone(),
                );
            }
            shrimply_support::crash::set_context("keyframe graph render end");
            glib::Propagation::Stop
        });
        graph.connect_unrealize(move |area| {
            shrimply_support::crash::set_context("keyframe graph unrealize begin");
            area.make_current();
            renderer.borrow_mut().destroy();
            shrimply_support::crash::set_context("keyframe graph unrealize end");
        });
    }

    let style = adw::StyleManager::for_display(&graph.display());
    let theme_graph = graph.clone();
    style.connect_dark_notify(move |_| theme_graph.queue_render());

    let motion = gtk::EventControllerMotion::new();
    {
        let graph = graph.clone();
        let pointer_pos = pointer_pos.clone();
        motion.connect_motion(move |_, x, y| {
            *pointer_pos.borrow_mut() = Some(vec2(x as f32, y as f32));
            graph.queue_render();
        });
    }
    {
        let graph = graph.clone();
        let pointer_pos = pointer_pos.clone();
        motion.connect_leave(move |_| {
            pointer_pos.borrow_mut().take();
            graph.queue_render();
        });
    }
    graph.add_controller(motion);

    let scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::BOTH_AXES);
    {
        let graph = graph.clone();
        let graph_view = graph_view.clone();
        let pointer_pos = pointer_pos.clone();
        let overscroll = overscroll.clone();
        let scrollbar_lifecycle = scrollbar_lifecycle.clone();
        let animation_tick_active = animation_tick_active.clone();
        scroll.connect_scroll(move |controller, dx, dy| {
            let width = graph.width().max(1) as f64;
            let height = graph.height().max(1) as f64;
            let item_range = graph_view.borrow().item_range();
            let pointer_x = pointer_pos
                .borrow()
                .map_or(width / 2.0, |pointer| f64::from(pointer.x));
            let ctrl = controller
                .current_event_state()
                .contains(gdk::ModifierType::CONTROL_MASK);
            let delta = if dx.abs() > f64::EPSILON { dx } else { dy };
            if !ctrl && delta.abs() > f64::EPSILON {
                let pointer = controller
                    .current_event()
                    .and_then(|event| event.position())
                    .map(|(x, y)| vec2(x as f32, y as f32))
                    .or_else(|| *pointer_pos.borrow());
                let mut view = graph_view.borrow_mut();
                view.initialize(item_range, width);
                view.clamp(item_range, width);
                if graph_scroll_should_propagate(*view, item_range, width, delta) {
                    if overscroll.borrow_mut().take().is_some() {
                        graph.queue_render();
                    }
                    return glib::Propagation::Proceed;
                }
                if let Some(scrollbar) = graph_scrollbar(*view, item_range, width, height) {
                    let mut scroll_seconds = view.scroll_seconds;
                    let event = scrollbar_lifecycle.borrow_mut().scroll_pages_at(
                        scrollbar,
                        pointer,
                        delta * GRAPH_SCROLLBAR_WHEEL_PAGE_FRACTION,
                        |value| scroll_seconds = item_range.0.as_secs_f64() + value,
                    );
                    if event.handled {
                        view.scroll_seconds = scroll_seconds;
                        view.clamp(item_range, width);
                        if event.animating {
                            start_graph_animation_tick(
                                &graph,
                                overscroll.clone(),
                                scrollbar_lifecycle.clone(),
                                animation_tick_active.clone(),
                            );
                        }
                        graph.queue_render();
                        return glib::Propagation::Stop;
                    }
                }
                drop(view);
            }
            let edge = update_graph_scroll(
                &mut graph_view.borrow_mut(),
                item_range,
                width,
                pointer_x,
                dx,
                dy,
                ctrl,
            );
            if update_graph_overscroll(&overscroll, edge) {
                start_graph_animation_tick(
                    &graph,
                    overscroll.clone(),
                    scrollbar_lifecycle.clone(),
                    animation_tick_active.clone(),
                );
            }
            graph.queue_render();
            glib::Propagation::Stop
        });
    }
    graph.add_controller(scroll);

    let click = gtk::GestureClick::new();
    click.set_button(1);
    {
        let graph = graph.clone();
        let graph_view = graph_view.clone();
        let project = project.clone();
        let selected_item = selected_item.clone();
        let select_time = select_time.clone();
        click.connect_released(move |_, _, x, y| {
            graph.grab_focus();
            if y <= CURSOR_LANE_HEIGHT {
                let item_range = graph_view.borrow().item_range();
                let domain = current_graph_domain(
                    &mut graph_view.borrow_mut(),
                    item_range,
                    graph.width().max(1) as f64,
                );
                let time = clamp_graph_time(
                    time_at_x(x, graph.width().max(1) as f64, domain),
                    item_range,
                );
                let Some(time) =
                    project_frame_keyframe_time(&project.borrow(), selected_item.as_ref(), time)
                else {
                    return;
                };
                select_time(time);
                graph.queue_render();
            }
        });
    }
    graph.add_controller(click);

    if actions.set_interpolation.is_some() || actions.text_interpolation.is_some() {
        let secondary_click = gtk::GestureClick::new();
        secondary_click.set_button(3);
        {
            let graph = graph.clone();
            let graph_data = graph_data.clone();
            let graph_view = graph_view.clone();
            secondary_click.connect_released(move |_, _, x, y| {
                let width = graph.width().max(1) as f64;
                let height = graph_content_height(graph.height().max(1) as f64);
                let item_range = graph_view.borrow().item_range();
                let domain = current_graph_domain(&mut graph_view.borrow_mut(), item_range, width);
                if let Some((owner_id, interpolation)) =
                    hit_graph_segment(&graph_data.borrow(), domain, width, height, x, y)
                {
                    let Some(set_interpolation) = actions.set_interpolation.clone() else {
                        return;
                    };
                    let graph_for_change = graph.clone();
                    let graph_data = graph_data.clone();
                    let changed = Rc::new(move |owner_id, interpolation| {
                        set_interpolation(owner_id, interpolation);
                        update_graph_interpolation(
                            &mut graph_data.borrow_mut(),
                            owner_id,
                            interpolation,
                        );
                        graph_for_change.queue_render();
                    }) as Rc<dyn Fn(Uuid, Interpolation)>;
                    show_interpolation_popover(
                        &graph,
                        x,
                        y,
                        interpolation,
                        owner_id,
                        Some(changed),
                        None,
                    );
                    return;
                }
                if y <= CURSOR_LANE_HEIGHT || y >= height {
                    return;
                }
                let Some((owner_id, interpolation)) =
                    graph_segment_at_x(&graph_data.borrow(), domain, width, x)
                else {
                    return;
                };
                let Some(text_interpolation) =
                    actions.text_interpolation.as_ref().and_then(|actions| {
                        (actions.get)(owner_id).map(|selected| (selected, actions.set.clone()))
                    })
                else {
                    return;
                };
                show_interpolation_popover(
                    &graph,
                    x,
                    y,
                    interpolation,
                    owner_id,
                    None,
                    Some(text_interpolation),
                );
            });
        }
        graph.add_controller(secondary_click);
    }

    let drag = gtk::GestureDrag::new();
    drag.set_button(1);
    {
        let active_target = active_target.clone();
        let graph = graph.clone();
        let graph_data = graph_data.clone();
        let graph_view = graph_view.clone();
        let add = add.clone();
        let key_selection = key_selection.clone();
        let selection_box = selection_box.clone();
        let selected_time = selected_time.clone();
        let project = project.clone();
        let selected_item = selected_item.clone();
        let select_time = select_time.clone();
        let scrollbar_lifecycle = scrollbar_lifecycle.clone();
        drag.connect_drag_begin(move |gesture, x, y| {
            graph.grab_focus();
            selection_box.borrow_mut().take();
            let width = graph.width().max(1) as f64;
            let height = graph.height().max(1) as f64;
            let content_height = graph_content_height(height);
            let item_range = graph_view.borrow().item_range();
            let mut view = graph_view.borrow_mut();
            let domain = current_graph_domain(&mut view, item_range, width);
            if let Some(scrollbar) = graph_scrollbar(*view, item_range, width, height) {
                let mut scroll_seconds = view.scroll_seconds;
                match scrollbar_lifecycle.borrow_mut().begin(
                    scrollbar,
                    vec2(x as f32, y as f32),
                    |value| {
                        scroll_seconds = item_range.0.as_secs_f64() + value;
                    },
                ) {
                    shrimply_skia_adw_ui::slider::Begin::None => {}
                    shrimply_skia_adw_ui::slider::Begin::Drag => {
                        view.scroll_seconds = scroll_seconds;
                        view.clamp(item_range, width);
                        *active_target.borrow_mut() = Some(DragTarget::SliderMove);
                        graph.queue_render();
                        return;
                    }
                }
            }
            drop(view);
            let graph_data = graph_data.borrow();
            let cursor = (y <= CURSOR_LANE_HEIGHT).then_some(DragTarget::Cursor);
            let point =
                hit_graph_point(&graph_data, domain, width, content_height, frame_step, x, y)
                    .map(DragTarget::Point);
            let target = cursor.or(point);
            drop(graph_data);
            if let Some(target) = target {
                let time = match target {
                    DragTarget::Point(point) => point.time,
                    DragTarget::Cursor => {
                        let Some(time) = project_frame_keyframe_time(
                            &project.borrow(),
                            selected_item.as_ref(),
                            clamp_graph_time(time_at_x(x, width, domain), item_range),
                        ) else {
                            return;
                        };
                        time
                    }
                    DragTarget::SelectBox => return,
                    DragTarget::SliderMove => return,
                };
                match target {
                    DragTarget::Cursor => {
                        select_time(time);
                        graph.queue_render();
                    }
                    DragTarget::Point(_) => {
                        *selected_time.borrow_mut() = Some(time);
                        let additive = gesture.current_event_state().intersects(
                            gdk::ModifierType::CONTROL_MASK | gdk::ModifierType::SHIFT_MASK,
                        );
                        if additive {
                            add_key_to_selection(&mut key_selection.borrow_mut(), time);
                        } else if !key_is_selected(&key_selection.borrow(), time) {
                            select_single_key(&mut key_selection.borrow_mut(), time);
                        } else {
                            key_selection.borrow_mut().focused = Some(time);
                        }
                        sync_keyframe_button(&add, Some(time));
                        graph.queue_render();
                    }
                    DragTarget::SelectBox | DragTarget::SliderMove => {}
                }
            } else if y > CURSOR_LANE_HEIGHT && y < content_height {
                let add_to_selection = gesture
                    .current_event_state()
                    .contains(gdk::ModifierType::CONTROL_MASK);
                if !add_to_selection {
                    set_key_selection(&mut key_selection.borrow_mut(), Vec::new(), None);
                    selected_time.borrow_mut().take();
                    sync_keyframe_button(&add, None);
                }
                *selection_box.borrow_mut() = Some(GraphSelectionBox {
                    start_x: x,
                    start_y: y,
                    end_x: x,
                    end_y: y,
                    add_to_selection,
                });
                *active_target.borrow_mut() = Some(DragTarget::SelectBox);
                graph.queue_render();
                return;
            }
            *active_target.borrow_mut() = target;
        });
    }
    {
        let active_target = active_target.clone();
        let graph = graph.clone();
        let graph_data = graph_data.clone();
        let graph_view = graph_view.clone();
        let add = add.clone();
        let key_selection = key_selection.clone();
        let selection_box = selection_box.clone();
        let selected_time = selected_time.clone();
        let update_point = actions.update_point.clone();
        let project = project.clone();
        let selected_item = selected_item.clone();
        let playhead = playhead.clone();
        let select_time = select_time.clone();
        let preferences = preferences.clone();
        let overscroll = overscroll.clone();
        let scrollbar_lifecycle = scrollbar_lifecycle.clone();
        let animation_tick_active = animation_tick_active.clone();
        drag.connect_drag_update(move |gesture, offset_x, offset_y| {
            let Some(target) = *active_target.borrow() else {
                return;
            };
            let Some((start_x, start_y)) = gesture.start_point() else {
                return;
            };
            let width = graph.width().max(1) as f64;
            let height = graph.height().max(1) as f64;
            let content_height = graph_content_height(height);
            if matches!(target, DragTarget::SliderMove) {
                let item_range = graph_view.borrow().item_range();
                let mut view = graph_view.borrow_mut();
                view.initialize(item_range, width);
                view.clamp(item_range, width);
                if let Some(scrollbar) = graph_scrollbar(*view, item_range, width, height) {
                    let mut scroll_seconds = view.scroll_seconds;
                    if scrollbar_lifecycle
                        .borrow_mut()
                        .drag_by(scrollbar, offset_x, |value| {
                            scroll_seconds = item_range.0.as_secs_f64() + value
                        })
                    {
                        view.scroll_seconds = scroll_seconds;
                        view.clamp(item_range, width);
                    }
                }
                graph.queue_render();
                return;
            }
            if matches!(target, DragTarget::Cursor) {
                let edge = {
                    let mut view = graph_view.borrow_mut();
                    let item_range = view.item_range();
                    let (time, edge) =
                        drag_cursor_time(&mut view, item_range, width, start_x + offset_x);
                    let Some(time) = project_frame_keyframe_time(
                        &project.borrow(),
                        selected_item.as_ref(),
                        time,
                    ) else {
                        return;
                    };
                    select_time(time);
                    edge
                };
                if update_graph_overscroll(&overscroll, edge) {
                    start_graph_animation_tick(
                        &graph,
                        overscroll.clone(),
                        scrollbar_lifecycle.clone(),
                        animation_tick_active.clone(),
                    );
                }
                graph.queue_render();
                return;
            }
            if matches!(target, DragTarget::SelectBox) {
                if let Some(selection) = selection_box.borrow_mut().as_mut() {
                    selection.end_x = start_x + offset_x;
                    selection.end_y = start_y + offset_y;
                    let graph_data = graph_data.borrow();
                    let item_range = graph_view.borrow().item_range();
                    let domain =
                        current_graph_domain(&mut graph_view.borrow_mut(), item_range, width);
                    let selected = select_keys_in_box(
                        &graph_data,
                        domain,
                        width,
                        content_height,
                        frame_step,
                        *selection,
                        &key_selection.borrow().selected,
                    );
                    let focused = selected.last().copied();
                    set_key_selection(&mut key_selection.borrow_mut(), selected, focused);
                    *selected_time.borrow_mut() = focused;
                    sync_keyframe_button(&add, focused);
                }
                graph.queue_render();
                return;
            }
            let mut point_updates = Vec::new();
            {
                let mut graph_data = graph_data.borrow_mut();
                let item_range = graph_view.borrow().item_range();
                let domain = current_graph_domain(&mut graph_view.borrow_mut(), item_range, width);
                let range = graph_range(&graph_data);
                let snap = preferences::snapshot(&preferences);
                let point_x = if matches!(*graph_data, KeyframeGraph::Step { .. }) {
                    start_x + offset_x
                        - shrimply_discrete_keyframe_graph_ui::frame_width(
                            width, domain, frame_step,
                        ) / 2.0
                } else {
                    start_x + offset_x
                };
                let time = snap_keyframe_time(
                    time_at_x(point_x, width, domain),
                    item_range,
                    snap.timeline_magnet == "true",
                    f64::from(snap.timeline_snap_radius_px),
                    graph_duration_seconds(domain) / graph_plot_width(width),
                    playhead(),
                );
                let Some(time) =
                    project_frame_keyframe_time(&project.borrow(), selected_item.as_ref(), time)
                else {
                    return;
                };
                let value = graph_edit_value(
                    &graph_data,
                    value_at_y(start_y + offset_y, content_height, range),
                );
                match target {
                    DragTarget::Cursor => {}
                    DragTarget::Point(point) => {
                        let selected_times = key_selection.borrow().selected.clone();
                        let (updates, next_selected, next_focus) = move_selected_graph_points(
                            &mut graph_data,
                            &selected_times,
                            point.time,
                            time,
                            value,
                            item_range,
                        );
                        point_updates = updates;
                        set_key_selection(
                            &mut key_selection.borrow_mut(),
                            next_selected,
                            Some(next_focus),
                        );
                        *selected_time.borrow_mut() = Some(next_focus);
                        sync_keyframe_button(&add, Some(next_focus));
                        *active_target.borrow_mut() =
                            graph_key_point(&graph_data, next_focus).map(DragTarget::Point);
                    }
                    DragTarget::SelectBox | DragTarget::SliderMove => {}
                }
            }
            for (old_time, time, value) in point_updates {
                update_point(old_time, time, value);
            }
            graph.queue_render();
        });
    }
    {
        let active_target = active_target.clone();
        let graph = graph.clone();
        let selection_box = selection_box.clone();
        let scrollbar_lifecycle = scrollbar_lifecycle.clone();
        drag.connect_drag_end(move |_, _, _| {
            scrollbar_lifecycle.borrow_mut().end_drag();
            selection_box.borrow_mut().take();
            active_target.borrow_mut().take();
            graph.queue_render();
        });
    }
    graph.add_controller(drag);

    let middle_drag = gtk::GestureDrag::new();
    middle_drag.set_button(2);
    {
        let graph = graph.clone();
        let graph_view = graph_view.clone();
        let scrollbar_lifecycle = scrollbar_lifecycle.clone();
        middle_drag.connect_drag_begin(move |_, x, _| {
            graph.grab_focus();
            let width = graph.width().max(1) as f64;
            let item_range = graph_view.borrow().item_range();
            scrollbar_lifecycle.borrow_mut().cancel_scroll();
            let mut view = graph_view.borrow_mut();
            view.initialize(item_range, width);
            view.clamp(item_range, width);
            view.drag_start_x = x;
            view.drag_start_scroll_seconds = view.scroll_seconds;
        });
    }
    {
        let graph = graph.clone();
        let graph_view = graph_view.clone();
        let overscroll = overscroll.clone();
        let scrollbar_lifecycle = scrollbar_lifecycle.clone();
        let animation_tick_active = animation_tick_active.clone();
        middle_drag.connect_drag_update(move |_, offset_x, _| {
            let width = graph.width().max(1) as f64;
            let edge = {
                let mut view = graph_view.borrow_mut();
                let target = view.drag_start_scroll_seconds - offset_x * view.seconds_per_pixel;
                let item_range = view.item_range();
                set_graph_scroll_seconds(&mut view, item_range, width, target)
            };
            if update_graph_overscroll(&overscroll, edge) {
                start_graph_animation_tick(
                    &graph,
                    overscroll.clone(),
                    scrollbar_lifecycle.clone(),
                    animation_tick_active.clone(),
                );
            }
            graph.queue_render();
        });
    }
    graph.add_controller(middle_drag);

    let key = gtk::EventControllerKey::new();
    key.set_propagation_phase(gtk::PropagationPhase::Capture);
    {
        let graph = graph.clone();
        let graph_data = graph_data.clone();
        let previous = previous.clone();
        let add = add.clone();
        let next = next.clone();
        let playhead = playhead.clone();
        let key_selection = key_selection.clone();
        let selected_time = selected_time.clone();
        let delete_at_time = actions.delete_at_time.clone();
        let copy_keyframes = actions.copy_keyframes.clone();
        let paste_keyframes = actions.paste_keyframes.clone();
        let toggle_playback = actions.toggle_playback.clone();
        let project = project.clone();
        let selected_item = selected_item.clone();
        key.connect_key_pressed(move |_, key, _, state| {
            if key == gdk::Key::space {
                toggle_playback();
                return glib::Propagation::Stop;
            }
            if state.contains(gdk::ModifierType::CONTROL_MASK) {
                match key.to_unicode().map(|key| key.to_ascii_lowercase()) {
                    Some('c') => {
                        let selected = key_selection.borrow().selected.clone();
                        if let Some(mut clipboard) = copy_keyframes(&selected) {
                            let project = project.borrow();
                            let timeline_times = clipboard
                                .times
                                .iter()
                                .map(|time| {
                                    selected_item
                                        .as_ref()
                                        .and_then(|item| {
                                            project.keyframe_timeline_time(item, *time)
                                        })
                                        .unwrap_or(*time)
                                        .snapped(project.frame_step())
                                })
                                .collect::<Vec<_>>();
                            let Some(origin) = timeline_times.first().copied() else {
                                return glib::Propagation::Stop;
                            };
                            clipboard.times = timeline_times
                                .into_iter()
                                .map(|time| Time {
                                    seconds: time.seconds - origin.seconds,
                                })
                                .collect();
                            drop(project);
                            let count = clipboard.len();
                            graph
                                .display()
                                .clipboard()
                                .set_text(KEYFRAME_CLIPBOARD_MARKER);
                            KEYFRAME_CLIPBOARD.with(|stored| stored.replace(Some(clipboard)));
                            let message = if count == 1 {
                                tr!("1 keyframe copied").into_owned()
                            } else {
                                shrimply_ui_foundation::i18n::text_args(
                                    "%{count} keyframes copied",
                                    &[("count", count.to_string())],
                                )
                            };
                            shrimply_ui_foundation::toast::show_confirmation_text_for_widget(
                                &graph, &message,
                            );
                        }
                        return glib::Propagation::Stop;
                    }
                    Some('v') => {
                        let clipboard = KEYFRAME_CLIPBOARD.with(|stored| stored.borrow().clone());
                        let Some(clipboard) = clipboard else {
                            return glib::Propagation::Stop;
                        };
                        let time = playhead();
                        let times = {
                            let project = project.borrow();
                            let anchor = selected_item
                                .as_ref()
                                .and_then(|item| project.keyframe_timeline_time(item, time))
                                .unwrap_or(time)
                                .snapped(project.frame_step());
                            clipboard
                                .times
                                .iter()
                                .filter_map(|offset| {
                                    let timeline_time = Time {
                                        seconds: anchor.seconds + offset.seconds,
                                    };
                                    selected_item
                                        .as_ref()
                                        .map(|item| project.keyframe_time(item, timeline_time))
                                        .unwrap_or(Some(timeline_time))
                                })
                                .collect::<Vec<_>>()
                        };
                        if times.len() != clipboard.len() {
                            return glib::Propagation::Stop;
                        }
                        let graph = graph.clone();
                        let paste_keyframes = paste_keyframes.clone();
                        let key_selection = key_selection.clone();
                        let selected_time = selected_time.clone();
                        graph.display().clipboard().read_text_async(
                            None::<&gio::Cancellable>,
                            move |result| {
                                let Some(text) = result.ok().flatten() else {
                                    return;
                                };
                                if text != KEYFRAME_CLIPBOARD_MARKER {
                                    return;
                                }
                                let Some(times) = paste_keyframes(&clipboard, &times) else {
                                    return;
                                };
                                let message = if times.len() == 1 {
                                    tr!("1 keyframe pasted").into_owned()
                                } else {
                                    shrimply_ui_foundation::i18n::text_args(
                                        "%{count} keyframes pasted",
                                        &[("count", times.len().to_string())],
                                    )
                                };
                                shrimply_ui_foundation::toast::show_confirmation_text_for_widget(
                                    &graph, &message,
                                );
                                let focus = times.first().copied();
                                set_key_selection(&mut key_selection.borrow_mut(), times, focus);
                                *selected_time.borrow_mut() = focus;
                                graph.queue_render();
                            },
                        );
                        return glib::Propagation::Stop;
                    }
                    _ => {}
                }
            }
            if !matches!(
                key,
                gdk::Key::BackSpace | gdk::Key::Delete | gdk::Key::KP_Delete
            ) {
                return glib::Propagation::Proceed;
            }
            let logical_playhead = playhead();
            let times = graph_data.borrow().key_times();
            let mut delete_times = key_selection.borrow().selected.clone();
            if delete_times.is_empty()
                && let Some(time) = keyframe_model::key_at(&times, logical_playhead, frame_step)
                    .or(*selected_time.borrow())
            {
                delete_times.push(time);
            }
            if delete_times.is_empty() {
                return glib::Propagation::Stop;
            }
            for time in &delete_times {
                delete_graph_key(&mut graph_data.borrow_mut(), *time);
            }
            set_key_selection(&mut key_selection.borrow_mut(), Vec::new(), None);
            selected_time.borrow_mut().take();
            sync_keyframe_controls(
                &previous,
                &add,
                &next,
                &graph_data.borrow(),
                logical_playhead,
                logical_playhead,
                frame_step,
            );
            graph.queue_render();
            for time in delete_times {
                delete_at_time(time);
            }
            glib::Propagation::Stop
        });
    }
    graph.add_controller(key.clone());

    {
        let previous_button = previous.clone();
        let graph = graph.clone();
        let graph_data = graph_data.clone();
        let add = add.clone();
        let next = next.clone();
        let playhead = playhead.clone();
        let selected_time = selected_time.clone();
        let select_time = select_time.clone();
        previous.connect_clicked(move |_| {
            let base_time = playhead();
            let time = previous_key_time(&graph_data.borrow(), base_time, frame_step);
            if let Some(time) = time {
                *selected_time.borrow_mut() = Some(time);
                sync_keyframe_controls(
                    &previous_button,
                    &add,
                    &next,
                    &graph_data.borrow(),
                    time,
                    time,
                    frame_step,
                );
                select_time(time);
                graph.grab_focus();
            }
        });
    }

    {
        let playhead = playhead.clone();
        let graph = graph.clone();
        let graph_data = graph_data.clone();
        let button = add.clone();
        let previous = previous.clone();
        let add = add.clone();
        let next = next.clone();
        let key_selection = key_selection.clone();
        let selected_time = selected_time.clone();
        let add_at_time = actions.add_at_time.clone();
        let delete_at_time = actions.delete_at_time.clone();
        let project = project.clone();
        let selected_item = selected_item.clone();
        button.connect_clicked(move |_| {
            let logical_playhead = playhead();
            let Some(actual_playhead) = project_frame_keyframe_time(
                &project.borrow(),
                selected_item.as_ref(),
                logical_playhead,
            ) else {
                return;
            };
            let times = graph_data.borrow().key_times();
            if let Some(time) = keyframe_model::key_at(&times, actual_playhead, frame_step) {
                delete_graph_key(&mut graph_data.borrow_mut(), time);
                set_key_selection(&mut key_selection.borrow_mut(), Vec::new(), None);
                selected_time.borrow_mut().take();
                sync_keyframe_controls(
                    &previous,
                    &add,
                    &next,
                    &graph_data.borrow(),
                    logical_playhead,
                    actual_playhead,
                    frame_step,
                );
                graph.queue_render();
                delete_at_time(time);
            } else {
                *selected_time.borrow_mut() = Some(actual_playhead);
                select_single_key(&mut key_selection.borrow_mut(), actual_playhead);
                add_at_time(actual_playhead);
            }
            graph.grab_focus();
        });
    }

    {
        let previous = previous.clone();
        let graph = graph.clone();
        let graph_data = graph_data.clone();
        let add = add.clone();
        let next_button = next.clone();
        let playhead = playhead.clone();
        let selected_time = selected_time.clone();
        let select_time = select_time.clone();
        next.connect_clicked(move |_| {
            let base_time = playhead();
            let time = next_key_time(&graph_data.borrow(), base_time, frame_step);
            if let Some(time) = time {
                *selected_time.borrow_mut() = Some(time);
                sync_keyframe_controls(
                    &previous,
                    &add,
                    &next_button,
                    &graph_data.borrow(),
                    time,
                    time,
                    frame_step,
                );
                select_time(time);
                graph.grab_focus();
            }
        });
    }

    root.append(&graph);

    let update_graph = {
        let graph = graph.clone();
        let graph_data = graph_data.clone();
        let previous = previous.clone();
        let add = add.clone();
        let next = next.clone();
        let playhead = playhead.clone();
        let graph_view = graph_view.clone();
        Rc::new(move |updated| {
            let visible_area =
                clip_bounded_visible_area(&project.borrow(), selected_item.as_ref(), visible_area);
            if let (Ok(mut graph_data), Ok(mut graph_view)) =
                (graph_data.try_borrow_mut(), graph_view.try_borrow_mut())
            {
                pending_graph.borrow_mut().take();
                *graph_data = updated;
                graph_view.visible_area = visible_area;
                let logical_playhead = playhead();
                sync_keyframe_controls(
                    &previous,
                    &add,
                    &next,
                    &graph_data,
                    logical_playhead,
                    logical_playhead,
                    frame_step,
                );
            } else {
                *pending_graph.borrow_mut() = Some((updated, visible_area));
            }
            graph.queue_render();
        }) as Rc<dyn Fn(KeyframeGraph)>
    };

    BuiltKeyframeEditor {
        widget: root.upcast(),
        update_graph,
    }
}

mod graph;

pub(crate) use graph::connect_graph_refresh_impl as connect_graph_refresh;
use graph::*;
