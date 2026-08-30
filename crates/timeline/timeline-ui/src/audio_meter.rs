use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::audio::SharedAudioLevels;
use adw::prelude::*;
use gtk::glib;
use shrimply_math_color::Color;

use super::renderer::{Align2, FontId, Rect, Stroke, TimelinePainter, TimelineRenderer, vec2};

const MIN_DB: f32 = -60.0;
const YELLOW_DB: f32 = -18.0;
const RED_DB: f32 = -6.0;
const RELEASE_DB_PER_SECOND: f32 = 24.0;
const PEAK_HOLD: Duration = Duration::from_millis(1_500);
const DEFAULT_WIDTH: i32 = 54;
const PADDING_LEFT: f32 = 6.0;
const PADDING_RIGHT: f32 = 7.0;
const METER_VERTICAL_PADDING: f32 = 16.0;
const RULER_WIDTH: f32 = 32.0;
const CHANNEL_GAP: f32 = 1.0;
const FONT_SIZE: f32 = 8.0;

#[derive(Clone, Copy)]
struct ChannelLevel {
    level_db: f32,
    peak_db: f32,
    peak_at: Instant,
}

struct AudioMeterRuntime {
    renderer: TimelineRenderer,
    channels: [ChannelLevel; 2],
    last_frame: Instant,
}

pub(super) fn new(levels: SharedAudioLevels) -> gtk::GLArea {
    let area = gtk::GLArea::builder()
        .auto_render(false)
        .has_depth_buffer(false)
        .has_stencil_buffer(false)
        .hexpand(true)
        .vexpand(true)
        .width_request(DEFAULT_WIDTH)
        .build();

    let now = Instant::now();
    let runtime = Rc::new(RefCell::new(AudioMeterRuntime {
        renderer: TimelineRenderer::new(),
        channels: [ChannelLevel {
            level_db: MIN_DB,
            peak_db: MIN_DB,
            peak_at: now,
        }; 2],
        last_frame: now,
    }));

    let render_runtime = runtime.clone();
    area.connect_render(move |area, _| {
        if let Some(error) = area.error() {
            tracing::error!("Audio meter GLArea error: {error}");
            return glib::Propagation::Stop;
        }
        area.make_current();
        if let Some(error) = area.error() {
            tracing::error!("Audio meter GLArea error after make_current: {error}");
            return glib::Propagation::Stop;
        }

        let width = area.width().max(1);
        let height = area.height().max(1);
        let pixels_per_point = area.scale_factor().max(1) as f32;
        let screen_size_px = glam::UVec2::new(
            (width as f32 * pixels_per_point).round().max(1.0) as u32,
            (height as f32 * pixels_per_point).round().max(1.0) as u32,
        );

        let mut runtime = render_runtime.borrow_mut();
        runtime.update(levels.take_peaks(), Instant::now());
        let channels = runtime.channels;
        let painter = match runtime.renderer.begin_frame(
            screen_size_px,
            pixels_per_point,
            crate::theme::view_bg(),
        ) {
            Ok(painter) => painter,
            Err(error) => {
                tracing::error!("Could not initialize Skia audio meter renderer: {error}");
                return glib::Propagation::Stop;
            }
        };
        draw_meter(&painter, width as f32, height as f32, channels);
        if let Err(error) = runtime.renderer.end_frame() {
            tracing::error!("Could not finalize Skia audio meter renderer: {error}");
        }

        glib::Propagation::Stop
    });

    area.add_tick_callback(|area, _| {
        area.queue_render();
        glib::ControlFlow::Continue
    });

    area.connect_unrealize(move |area| {
        area.make_current();
        runtime.borrow_mut().renderer.destroy();
    });

    area
}

impl AudioMeterRuntime {
    fn update(&mut self, peaks: [f32; 2], now: Instant) {
        let release = RELEASE_DB_PER_SECOND * now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        for (channel, amplitude) in self.channels.iter_mut().zip(peaks) {
            let level = amplitude_to_db(amplitude);
            channel.level_db = if level >= channel.level_db {
                level
            } else {
                (channel.level_db - release).max(level).max(MIN_DB)
            };

            if level >= channel.peak_db {
                channel.peak_db = level;
                channel.peak_at = now;
            } else if now.duration_since(channel.peak_at) >= PEAK_HOLD {
                channel.peak_db = (channel.peak_db - release)
                    .max(channel.level_db)
                    .max(MIN_DB);
            }
        }
    }
}

fn amplitude_to_db(amplitude: f32) -> f32 {
    if amplitude <= 0.0 {
        MIN_DB
    } else {
        (20.0 * amplitude.log10()).clamp(MIN_DB, 0.0)
    }
}

fn draw_meter(painter: &TimelinePainter, width: f32, height: f32, channels: [ChannelLevel; 2]) {
    let top = METER_VERTICAL_PADDING.min((height - 1.0) / 2.0);
    let bottom = (height - METER_VERTICAL_PADDING).max(top + 1.0);
    let bars_right = (width - PADDING_RIGHT - RULER_WIDTH).max(PADDING_LEFT + CHANNEL_GAP + 2.0);
    let meter = Rect::from_min_size(
        vec2(PADDING_LEFT, top),
        vec2(bars_right - PADDING_LEFT, bottom - top),
    );
    painter.rect_filled(meter, 1.0, Color::DARK5);

    let bar_left = meter.left();
    let bar_top = meter.top();
    let bar_bottom = meter.bottom();
    let bars_width = (meter.right() - bar_left - CHANNEL_GAP).max(2.0);
    let channel_width = bars_width / 2.0;
    let bars = [
        Rect::from_min_size(
            vec2(bar_left, bar_top),
            vec2(channel_width, bar_bottom - bar_top),
        ),
        Rect::from_min_size(
            vec2(bar_left + channel_width + CHANNEL_GAP, bar_top),
            vec2(channel_width, bar_bottom - bar_top),
        ),
    ];

    for (bar, channel) in bars.into_iter().zip(channels) {
        draw_level_band(
            painter,
            bar,
            channel.level_db,
            MIN_DB,
            YELLOW_DB,
            Color::GREEN3,
        );
        draw_level_band(
            painter,
            bar,
            channel.level_db,
            YELLOW_DB,
            RED_DB,
            Color::YELLOW3,
        );
        draw_level_band(painter, bar, channel.level_db, RED_DB, 0.0, Color::RED3);
        if channel.peak_db > MIN_DB {
            let peak_y = db_y(channel.peak_db, bar.top(), bar.bottom());
            let peak_color = if channel.peak_db >= RED_DB {
                Color::RED1
            } else {
                Color::YELLOW1
            };
            painter.rect_filled(
                Rect::from_min_size(vec2(bar.left(), peak_y - 1.0), vec2(bar.width(), 2.0)),
                0,
                peak_color,
            );
        }
    }

    draw_ruler(painter, bars_right, width, meter.top(), meter.bottom());
}

fn draw_level_band(
    painter: &TimelinePainter,
    bar: Rect,
    level_db: f32,
    low_db: f32,
    high_db: f32,
    color: Color,
) {
    let active_high = level_db.min(high_db);
    if active_high <= low_db {
        return;
    }
    let top = db_y(active_high, bar.top(), bar.bottom());
    let bottom = db_y(low_db, bar.top(), bar.bottom());
    painter.rect_filled(
        Rect::from_min_size(vec2(bar.left(), top), vec2(bar.width(), bottom - top)),
        0,
        color,
    );
}

fn draw_ruler(painter: &TimelinePainter, bars_right: f32, width: f32, top: f32, bottom: f32) {
    let height = bottom - top;
    let step = if height * 3.0 / -MIN_DB >= 14.0 {
        3
    } else if height * 6.0 / -MIN_DB >= 14.0 {
        6
    } else {
        12
    };
    let tick_start = bars_right + 5.0;
    let tick_end = tick_start + 3.0;
    let label_gap = 4.0;
    let font = FontId::proportional(FONT_SIZE);

    for db in (0..=60).step_by(step) {
        let y = db_y(-(db as f32), top, bottom);
        painter.line_segment(
            [vec2(tick_start, y), vec2(tick_end, y)],
            Stroke::new(1.0, Color::DARK1),
        );
        let label = if db == 0 {
            String::from("0")
        } else {
            format!("-{db}")
        };
        let size = painter
            .layout_no_wrap(&label, font.clone(), Color::LIGHT4)
            .size();
        if tick_end + label_gap + size.x <= width - PADDING_RIGHT {
            painter.text(
                vec2(tick_end + label_gap, y - size.y / 2.0),
                Align2::LEFT_TOP,
                label,
                font.clone(),
                Color::LIGHT4,
            );
        }
    }
}

fn db_y(db: f32, top: f32, bottom: f32) -> f32 {
    top + (-db.clamp(MIN_DB, 0.0) / -MIN_DB) * (bottom - top)
}
