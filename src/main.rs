
use valora::prelude::*;
use num_traits::Float;
use std::f64;
use chrono::prelude::*;


#[warn(deprecated)]
fn main() -> Result<()> {
    run_fn(Options::from_args(), |_gpu, world, _rng| {
        Ok(move |ctx: Context, canvas: &mut Canvas| {
            draw_clock(canvas, world, ctx);
        })
    })
}

fn draw_clock(canvas: &mut Canvas, world:World, ctx:Context){
    canvas.set_color(LinSrgb::new(1., 1., 1.));
    canvas.paint(Filled(ctx.world));
 
 
    let max_radius = world.width / 3.;
 
 
    canvas.set_color(LinSrgb::new(0., 0., 0.));
    canvas.paint(Stroked{
        width: 3.,
        element:Ellipse::circle(world.center(), max_radius)
    });
    draw_text(canvas, world);
    draw_hand(canvas, ctx);
}

fn draw_hand(canvas: &mut Canvas, ctx:Context) {
    
    let center = ctx.world.center();
    let sys_time = Local::now();
    let radius:f64 = (ctx.world.width / 3.) as f64;
    let hour = (sys_time.hour()*30 + (sys_time.minute() as f32/60.*30.) as u32) as f64;
    let min = (sys_time.minute()*6) as f64;
    let second = (sys_time.second()*6) as f64;

    let mut de = ((hour)/180.)*f64::consts::PI;
    canvas.move_to(ctx.world.center());
    canvas.line_to(P2::new((center.x as f64+(radius-50.)*(de.sin())) as f32, (center.y as f64+(radius-50.)*(de.cos())) as f32));
    canvas.set_stroke_width(7.);
    canvas.stroke();
    
    de = ((min)/180.)*f64::consts::PI;
    canvas.move_to(ctx.world.center());
    canvas.line_to(P2::new((center.x as f64+(radius-40.)*(de.sin())) as f32, (center.y as f64+(radius-40.)*(de.cos())) as f32));
    canvas.set_stroke_width(5.);
    canvas.stroke();

    de = ((second)/180.)*f64::consts::PI;
    canvas.move_to(ctx.world.center());
    canvas.line_to(P2::new((center.x as f64+(radius-30.)*(de.sin())) as f32, (center.y as f64+(radius-30.)*(de.cos())) as f32));
    canvas.set_stroke_width(3.);
    canvas.stroke();

}

fn draw_text(canvas: &mut Canvas, world:World) {
    let degree:f32 = 360./60.;
    let radius:f64 = (world.width / 3.) as f64;
    let center = world.center();

    for count in 0..60{
        if count%5 == 0 {
            //绘制时坐标
            let de = ((count as f32*degree)/180.) as f64*f64::consts::PI;
            let startx = center.x as f64+radius*(de.sin());
            let starty = center.y as f64+radius*(de.cos());
            let endx = center.x as f64+(radius-20.)*(de.sin());
            let endy = center.y as f64+(radius-20.)*(de.cos());
            //println!("x: {:?}, y:{:?}", startx, starty);
            canvas.move_to(P2::new(startx as f32, starty as f32));
            canvas.line_to(P2::new(endx as f32, endy as f32));
            canvas.set_stroke_width(10.);
            canvas.stroke();
        }
        else {
            //绘制分坐标
            let de = ((count as f32*degree)/180.) as f64*f64::consts::PI;
            let startx = center.x as f64+radius*(de.sin());
            let starty = center.y as f64+radius*(de.cos());
            let endx = center.x as f64+(radius-10.)*(de.sin());
            let endy = center.y as f64+(radius-10.)*(de.cos());
            //println!("x: {:?}, y:{:?}", startx, starty);
            canvas.move_to(P2::new(startx as f32, starty as f32));
            canvas.line_to(P2::new(endx as f32, endy as f32));
            canvas.set_stroke_width(3.);
            canvas.stroke();
        }
    }
}

