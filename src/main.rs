//! An almost exact copy of the example of a custom drawing widget from Druid itself
//! We plan to draw tablature

use druid::kurbo::{BezPath, Line};
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc,
};

struct TablatureWidget;

impl TablatureWidget {
    pub const STRINGCOLOR: Color = Color::rgb8(0, 128, 0);
}

// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
impl Widget<String> for TablatureWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut String, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &String,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &String,
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
    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        let line_delta = size.height / 7.0;
        let mut line_y = line_delta; // Kinda hack, should be neck margin

        for _i in 0..6 {
            ctx.stroke(
                Line::new((0.0, line_y), (size.width, line_y)),
                &Self::STRINGCOLOR,
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
        let mut layout = TextLayout::<String>::from_text(data);
        layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(24.0));
        layout.set_text_color(fill_color);
        layout.rebuild_if_needed(ctx.text(), env);

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
            .new_text_layout(data.clone())
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
}

pub fn main() {
    let window = WindowDesc::new(|| TablatureWidget {}).title(LocalizedString::new("fret"));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch("Druid + Piet + Bleeding Fingers".to_string())
        .expect("launch failed");
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
