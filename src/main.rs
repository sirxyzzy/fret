//! An almost exact copy of the example of a custom drawing widget from Druid itself
//! We plan to draw tablature
use core::f64;

use druid::kurbo::Line;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Rect, TextLayout, WindowDesc,
};

use std::sync::Arc;

use num::rational::Rational32;

struct FretNumber(u32);

struct StringNumber(u32);

/// How many beats in a bar, so 4, for 4:4 time and two for 2:4 time
struct BeatsPerBar(u32);

/// The duration of one beat, usually a power of two, a crotchet for 2:4 or 4:4
struct NoteValue(u32);

struct Signature {
    bpb: BeatsPerBar,
    value: NoteValue,
}

/// An ugly little helper, approximate a rational
/// by a float, this is potentially lossy, consider 1/3 => 1.33333...
fn as_float(r: &Rational32) -> f64 {
    (*(r.numer()) as f64) / (*(r.denom()) as f64)
}

/// The units here are beats, we allow for fractional positions, such as half a beat
/// and for accuracy store that as a private ratio n/d
struct BarPosition(Rational32);

impl BarPosition {
    fn new() -> BarPosition {
        BarPosition(num::zero())
    }

    fn from_ratio(n: i32, d: i32) -> BarPosition {
        BarPosition(Rational32::new(n, d))
    }

    fn as_float(&self) -> f64 {
        as_float(&self.0)
    }
}

/// A notation gives a fret, a string, and a bar position
enum Action {
    Unused(),
    Simple(BarPosition),
    Muted(),
}
struct Notation {
    fret: FretNumber,
    action: Action,
}
struct Chord {
    notations: Vec<Notation>, // One per string
    position: BarPosition,
}

struct Bar {
    size: f64, // relative, 1.0 == default, 2.0 == twice default, etc...
    signature: Signature,
    notations: Vec<Notation>,
}

#[derive(Clone, Data)]
struct AppData {
    bars: Arc<Vec<Bar>>,
}

impl AppData {
    pub fn new() -> AppData {
        AppData {
            bars: Arc::new(Vec::new()),
        }
    }
}

struct TablatureWidget {}

impl TablatureWidget {
    pub const STRING_COLOR: Color = Color::rgb8(0, 128, 0);

    pub fn new() -> TablatureWidget {
        TablatureWidget {}
    }
}

// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
impl Widget<AppData> for TablatureWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppData, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppData,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() | bc.is_height_bounded() {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        } else {
            bc.max()
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        for bar in data.bars.iter() {}

        let line_delta = size.height / 7.0;
        let mut line_y = line_delta; // Kinda hack, should be neck margin

        for _i in 0..6 {
            ctx.stroke(
                Line::new((0.0, line_y), (size.width, line_y)),
                &Self::STRING_COLOR,
                4.0,
            );
            line_y += line_delta;
        }

        // Rectangles: the path for practical people
        let nut_rect = Rect::new(0.0, 0.0, line_delta / 5.0, size.height);
        // My nuts are black
        let fill_color = Color::rgb8(0x00, 0x00, 0x00);
        ctx.fill(nut_rect, &fill_color);

        // Text is easy; in real use TextLayout should either be stored in the
        // widget and reused, or a label child widget to manage it all.
        // This is one way of doing it, you can also use a builder-style way.
        let mut layout = new_text_layout("Whoops no data");
        layout.rebuild_if_needed(ctx.text(), env);
        for i in 0..10 {
            let x = i as f64;
            layout.draw(ctx, (80.0 + (x * 10.0), 40.0 + (x * 15.0)));
        }

        // Let's rotate our text slightly. First we save our current (default) context:
        ctx.with_save(|ctx| {
            // Now we can rotate the context (or set a clip path, for instance):
            // This makes it so that anything drawn after this (in the closure) is
            // transformed.
            // The transformation is in radians, but be aware it transforms the canvas,
            // not just the part you are drawing. So we draw at (80.0, 40.0) on the rotated
            // canvas, this is NOT the same position as (80.0, 40.0) on the original canvas.
            ctx.transform(Affine::rotate(std::f64::consts::FRAC_PI_4));
            layout.draw(ctx, (80.0, 40.0));
        });
        // When we exit with_save, the original context's rotation is restored

        // This is the builder-style way of drawing text.
        let text = ctx.text();
        let layout = text
            .new_text_layout("Still no data!!")
            .font(FontFamily::SERIF, 24.0)
            .text_color(Color::rgb8(128, 0, 0))
            .build()
            .unwrap();
        ctx.draw_text(&layout, (100.0, 25.0));

        // Let's burn some CPU to make a (partially transparent) image buffer
        let image_data = make_image_data(256, 256);
        let image = ctx
            .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
            .unwrap();
        // The image is automatically scaled to fit the rect you pass to draw_image
        ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);
    }

    // // The paint method gets called last, after an event flow.
    // // It goes event -> update -> layout -> paint, and each method can influence the next.
    // // Basically, anything that changes the appearance of a widget causes a paint.
    // fn _paint_old(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
    //     // Clear the whole widget with the color of your choice
    //     // (ctx.size() returns the size of the layout rect we're painting in)
    //     // Note: ctx also has a `clear` method, but that clears the whole context,
    //     // and we only want to clear this widget's area.
    //     let size = ctx.size();
    //     let rect = size.to_rect();
    //     ctx.fill(rect, &Color::WHITE);

    //     let line_delta = size.height / 7.0;
    //     let mut line_y = line_delta; // Kinda hack, should be neck margin

    //     for _i in 0..6 {
    //         ctx.stroke(
    //             Line::new((0.0, line_y), (size.width, line_y)),
    //             &Self::STRING_COLOR,
    //             4.0,
    //         );
    //         line_y += line_delta;
    //     }

    //     // Rectangles: the path for practical people
    //     let nut_rect = Rect::new(0.0, 0.0, line_delta / 5.0, size.height);
    //     // My nuts are black
    //     let fill_color = Color::rgb8(0x00, 0x00, 0x00);
    //     ctx.fill(nut_rect, &fill_color);

    //     // Text is easy; in real use TextLayout should either be stored in the
    //     // widget and reused, or a label child widget to manage it all.
    //     // This is one way of doing it, you can also use a builder-style way.
    //     let mut layout = new_text_layout(data);
    //     layout.rebuild_if_needed(ctx.text(), env);
    //     for i in 0..10 {
    //         let x = i as f64;
    //         layout.draw(ctx, (80.0 + (x * 10.0), 40.0 + (x * 15.0)));
    //     }

    //     // Let's rotate our text slightly. First we save our current (default) context:
    //     ctx.with_save(|ctx| {
    //         // Now we can rotate the context (or set a clip path, for instance):
    //         // This makes it so that anything drawn after this (in the closure) is
    //         // transformed.
    //         // The transformation is in radians, but be aware it transforms the canvas,
    //         // not just the part you are drawing. So we draw at (80.0, 40.0) on the rotated
    //         // canvas, this is NOT the same position as (80.0, 40.0) on the original canvas.
    //         ctx.transform(Affine::rotate(std::f64::consts::FRAC_PI_4));
    //         layout.draw(ctx, (80.0, 40.0));
    //     });
    //     // When we exit with_save, the original context's rotation is restored

    //     // This is the builder-style way of drawing text.
    //     let text = ctx.text();
    //     let layout = text
    //         .new_text_layout(data.clone())
    //         .font(FontFamily::SERIF, 24.0)
    //         .text_color(Color::rgb8(128, 0, 0))
    //         .build()
    //         .unwrap();
    //     ctx.draw_text(&layout, (100.0, 25.0));

    //     // Let's burn some CPU to make a (partially transparent) image buffer
    //     let image_data = make_image_data(256, 256);
    //     let image = ctx
    //         .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
    //         .unwrap();
    //     // The image is automatically scaled to fit the rect you pass to draw_image
    //     ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);
    // }
}

pub fn main() {
    let window = WindowDesc::new(TablatureWidget::new()).title(LocalizedString::new("fret"));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(AppData::new())
        .expect("launch failed");
}

fn new_text_layout(data: &str) -> TextLayout<String> {
    let fill_color = Color::rgb8(0x00, 0x00, 0x00);
    let mut layout = TextLayout::<String>::from_text(data);
    layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(24.0));
    layout.set_text_color(fill_color);
    layout
}

fn make_image_data(width: usize, height: usize) -> Vec<u8> {
    let mut result = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            result[ix] = x as u8;
            result[ix + 1] = y as u8;
            result[ix + 2] = !(x as u8);
            result[ix + 3] = 127;
        }
    }
    result
}
