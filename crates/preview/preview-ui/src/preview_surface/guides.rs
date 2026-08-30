use super::*;
use crate::project::PreviewGuides;
use crate::timeline::renderer::{Align2, FontId, Stroke, TimelinePainter, Vec2};
use shrimply_preview_core::PreviewViewport;

const RULER_SIZE_PX: f32 = 24.0;
pub(super) const MIN_PADDING_PX: u32 = RULER_SIZE_PX as u32;
const GUIDE_HIT_RADIUS_PX: f32 = 5.0;
const RULER_DIVISIONS: usize = 20;
const MAJOR_TICK_INTERVAL: usize = 5;

#[derive(Clone, Copy)]
pub(super) enum GuideDrag {
    Vertical { index: usize, original: Option<f32> },
    Horizontal { index: usize, original: Option<f32> },
}

impl GuideDrag {
    pub(super) fn begin(
        guides: &mut PreviewGuides,
        viewport: PreviewViewport,
        position: GlamVec2,
    ) -> Option<Self> {
        let drag = match ruler_axis(position) {
            Some(GuideAxis::Vertical) => {
                guides.vertical.push(0.0);
                Self::Vertical {
                    index: guides.vertical.len() - 1,
                    original: None,
                }
            }
            Some(GuideAxis::Horizontal) => {
                guides.horizontal.push(0.0);
                Self::Horizontal {
                    index: guides.horizontal.len() - 1,
                    original: None,
                }
            }
            None => return Self::existing(guides, viewport, position),
        };
        drag.update(guides, viewport, position);
        Some(drag)
    }

    pub(super) fn update(
        self,
        guides: &mut PreviewGuides,
        viewport: PreviewViewport,
        position: GlamVec2,
    ) {
        let canvas = viewport.screen_point_to_canvas(position);
        match self {
            Self::Vertical { index, .. } => {
                guides.vertical[index] = canvas.x.clamp(0.0, viewport.canvas_size.x);
            }
            Self::Horizontal { index, .. } => {
                guides.horizontal[index] = canvas.y.clamp(0.0, viewport.canvas_size.y);
            }
        }
    }

    pub(super) fn cancel(self, guides: &mut PreviewGuides) {
        match self {
            Self::Vertical { index, original } => match original {
                Some(value) => guides.vertical[index] = value,
                None => {
                    guides.vertical.remove(index);
                }
            },
            Self::Horizontal { index, original } => match original {
                Some(value) => guides.horizontal[index] = value,
                None => {
                    guides.horizontal.remove(index);
                }
            },
        }
    }

    fn remove(self, guides: &mut PreviewGuides) {
        match self {
            Self::Vertical { index, .. } => {
                guides.vertical.remove(index);
            }
            Self::Horizontal { index, .. } => {
                guides.horizontal.remove(index);
            }
        }
    }

    pub(super) const fn cursor(self) -> &'static str {
        match self {
            Self::Vertical { .. } => "ew-resize",
            Self::Horizontal { .. } => "ns-resize",
        }
    }

    const fn is_new(self) -> bool {
        match self {
            Self::Vertical { original, .. } | Self::Horizontal { original, .. } => {
                original.is_none()
            }
        }
    }

    const fn returned_to_edge(self, position: GlamVec2) -> bool {
        match self {
            Self::Vertical { .. } => position.x < RULER_SIZE_PX,
            Self::Horizontal { .. } => position.y < RULER_SIZE_PX,
        }
    }

    fn existing(
        guides: &PreviewGuides,
        viewport: PreviewViewport,
        position: GlamVec2,
    ) -> Option<Self> {
        if position.x < RULER_SIZE_PX || position.y < RULER_SIZE_PX {
            return None;
        }
        let mut closest = None;
        for (index, value) in guides.vertical.iter().copied().enumerate() {
            let screen = viewport.canvas_point_to_screen(GlamVec2::new(value, 0.0)).x;
            let distance = (position.x - screen).abs();
            if distance <= GUIDE_HIT_RADIUS_PX && closest.is_none_or(|(best, _)| distance < best) {
                closest = Some((
                    distance,
                    Self::Vertical {
                        index,
                        original: Some(value),
                    },
                ));
            }
        }
        for (index, value) in guides.horizontal.iter().copied().enumerate() {
            let screen = viewport.canvas_point_to_screen(GlamVec2::new(0.0, value)).y;
            let distance = (position.y - screen).abs();
            if distance <= GUIDE_HIT_RADIUS_PX && closest.is_none_or(|(best, _)| distance < best) {
                closest = Some((
                    distance,
                    Self::Horizontal {
                        index,
                        original: Some(value),
                    },
                ));
            }
        }
        closest.map(|(_, drag)| drag)
    }
}

pub(super) fn begin_drag(
    area: &gtk::GLArea,
    project: &Rc<RefCell<Project>>,
    state: &Rc<RefCell<VideoSurfaceState>>,
    controller: &Rc<RefCell<PreviewControllerState>>,
    position: GlamVec2,
) -> Option<GuideDrag> {
    if !state.borrow().guides_visible {
        return None;
    }
    let mut project = project.borrow_mut();
    let viewport = surface_viewport(area, &project, &state.borrow());
    let drag = GuideDrag::begin(&mut project.preview_guides, viewport, position)?;
    controller.borrow_mut().sequence = PointerSequence::Guide;
    area.set_cursor_from_name(Some(drag.cursor()));
    area.queue_render();
    Some(drag)
}

pub(super) fn update_drag(
    area: &gtk::GLArea,
    project: &Rc<RefCell<Project>>,
    state: &Rc<RefCell<VideoSurfaceState>>,
    drag: GuideDrag,
    position: GlamVec2,
) {
    let mut project = project.borrow_mut();
    let viewport = surface_viewport(area, &project, &state.borrow());
    drag.update(&mut project.preview_guides, viewport, position);
    drop(project);
    area.queue_render();
}

pub(super) fn finish_drag(
    area: &gtk::GLArea,
    project: &Rc<RefCell<Project>>,
    state: &Rc<RefCell<VideoSurfaceState>>,
    controller: &Rc<RefCell<PreviewControllerState>>,
    drag: GuideDrag,
    position: GlamVec2,
    moved: bool,
) {
    let mut project_state = project.borrow_mut();
    let changed = if drag.returned_to_edge(position) {
        if drag.is_new() {
            drag.cancel(&mut project_state.preview_guides);
            false
        } else {
            drag.remove(&mut project_state.preview_guides);
            true
        }
    } else if moved {
        let viewport = surface_viewport(area, &project_state, &state.borrow());
        drag.update(&mut project_state.preview_guides, viewport, position);
        true
    } else {
        drag.cancel(&mut project_state.preview_guides);
        false
    };
    if changed {
        drop(project_state);
        crate::project::commit_edit(&project.borrow(), "preview-guide");
    }
    let mut controller = controller.borrow_mut();
    controller.sequence = PointerSequence::Idle;
    controller.context_invalidated = changed;
    drop(controller);
    area.set_cursor_from_name(None);
    area.queue_render();
}

pub(super) fn cancel_drag(
    area: &gtk::GLArea,
    project: &Rc<RefCell<Project>>,
    controller: &Rc<RefCell<PreviewControllerState>>,
    drag: GuideDrag,
) {
    drag.cancel(&mut project.borrow_mut().preview_guides);
    controller.borrow_mut().sequence = PointerSequence::Idle;
    area.set_cursor_from_name(None);
    area.queue_render();
}

#[derive(Clone, Copy)]
enum GuideAxis {
    Vertical,
    Horizontal,
}

fn ruler_axis(position: GlamVec2) -> Option<GuideAxis> {
    if position.y < RULER_SIZE_PX && position.x >= RULER_SIZE_PX {
        Some(GuideAxis::Horizontal)
    } else if position.x < RULER_SIZE_PX && position.y >= RULER_SIZE_PX {
        Some(GuideAxis::Vertical)
    } else {
        None
    }
}

pub(super) fn ruler_cursor(position: GlamVec2) -> Option<&'static str> {
    match ruler_axis(position)? {
        GuideAxis::Vertical => Some("ew-resize"),
        GuideAxis::Horizontal => Some("ns-resize"),
    }
}

pub(super) fn hover_cursor(
    area: &gtk::GLArea,
    project: &Rc<RefCell<Project>>,
    state: &Rc<RefCell<VideoSurfaceState>>,
    position: GlamVec2,
) -> Option<&'static str> {
    if !state.borrow().guides_visible {
        return None;
    }
    if let Some(cursor) = ruler_cursor(position) {
        return Some(cursor);
    }
    let project = project.borrow();
    let viewport = surface_viewport(area, &project, &state.borrow());
    GuideDrag::existing(&project.preview_guides, viewport, position).map(GuideDrag::cursor)
}

pub(super) fn draw(
    painter: &TimelinePainter,
    guides: &PreviewGuides,
    canvas_size: GlamVec2,
    content_rect: Rect,
    surface_rect: Rect,
    color: Color,
) {
    let viewport = PreviewViewport::new(canvas_size, content_rect);
    for guide in &guides.vertical {
        let x = viewport
            .canvas_point_to_screen(GlamVec2::new(*guide, 0.0))
            .x;
        line(
            painter,
            vec2(x, surface_rect.top() + RULER_SIZE_PX),
            vec2(x, surface_rect.bottom()),
            color,
        );
    }
    for guide in &guides.horizontal {
        let y = viewport
            .canvas_point_to_screen(GlamVec2::new(0.0, *guide))
            .y;
        line(
            painter,
            vec2(surface_rect.left() + RULER_SIZE_PX, y),
            vec2(surface_rect.right(), y),
            color,
        );
    }
    draw_rulers(painter, viewport, content_rect, surface_rect);
}

fn draw_rulers(
    painter: &TimelinePainter,
    viewport: PreviewViewport,
    content_rect: Rect,
    surface_rect: Rect,
) {
    let background = shrimply_skia_adw_ui::theme::current().sidebar_bg;
    let foreground = shrimply_skia_adw_ui::theme::current().sidebar_fg;
    painter.rect_filled(
        Rect::from_min_size(
            vec2(surface_rect.left(), surface_rect.top()),
            vec2(surface_rect.width(), RULER_SIZE_PX),
        ),
        0,
        background,
    );
    painter.rect_filled(
        Rect::from_min_size(
            vec2(surface_rect.left(), surface_rect.top() + RULER_SIZE_PX),
            vec2(
                RULER_SIZE_PX,
                (surface_rect.height() - RULER_SIZE_PX).max(0.0),
            ),
        ),
        0,
        background,
    );
    let step_x = viewport.canvas_size.x / RULER_DIVISIONS as f32;
    let first_x = viewport
        .screen_point_to_canvas(GlamVec2::new(surface_rect.left(), content_rect.top()))
        .x
        .div_euclid(step_x) as i32;
    let last_x = (viewport
        .screen_point_to_canvas(GlamVec2::new(surface_rect.right(), content_rect.top()))
        .x
        / step_x)
        .ceil() as i32;
    for index in first_x..=last_x {
        let major = index.rem_euclid(MAJOR_TICK_INTERVAL as i32) == 0;
        let tick = if major { 9.0 } else { 5.0 };
        let value = index as f32 * step_x;
        let screen = viewport.canvas_point_to_screen(GlamVec2::new(value, 0.0));
        if screen.x <= surface_rect.left() + RULER_SIZE_PX {
            continue;
        }
        painter.line_segment(
            [
                vec2(screen.x, RULER_SIZE_PX - tick),
                vec2(screen.x, RULER_SIZE_PX),
            ],
            Stroke::new(1.0, foreground),
        );
        if major {
            painter.text(
                vec2(screen.x + 2.0, 2.0),
                Align2::LEFT_TOP,
                format!("{value:.0}"),
                FontId::proportional(9.0),
                foreground,
            );
        }
    }

    let step_y = viewport.canvas_size.y / RULER_DIVISIONS as f32;
    let first_y = viewport
        .screen_point_to_canvas(GlamVec2::new(content_rect.left(), surface_rect.top()))
        .y
        .div_euclid(step_y) as i32;
    let last_y = (viewport
        .screen_point_to_canvas(GlamVec2::new(content_rect.left(), surface_rect.bottom()))
        .y
        / step_y)
        .ceil() as i32;
    for index in first_y..=last_y {
        let major = index.rem_euclid(MAJOR_TICK_INTERVAL as i32) == 0;
        let tick = if major { 9.0 } else { 5.0 };
        let value = index as f32 * step_y;
        let screen = viewport.canvas_point_to_screen(GlamVec2::new(0.0, value));
        if screen.y <= surface_rect.top() + RULER_SIZE_PX {
            continue;
        }
        painter.line_segment(
            [
                vec2(RULER_SIZE_PX - tick, screen.y),
                vec2(RULER_SIZE_PX, screen.y),
            ],
            Stroke::new(1.0, foreground),
        );
        if major {
            painter.text_rotated(
                vec2(RULER_SIZE_PX - 3.0, screen.y + 2.0),
                Align2::LEFT_TOP,
                format!("{value:.0}"),
                FontId::proportional(9.0),
                foreground,
                90.0,
            );
        }
    }
}

fn line(painter: &TimelinePainter, start: Vec2, end: Vec2, color: Color) {
    painter.line_segment([start, end], Stroke::new(1.5, color));
}
