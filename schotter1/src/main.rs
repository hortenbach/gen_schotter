use nannou::prelude::*;

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;

fn main() {
    nannou::sketch(view)
        .size(WIDTH, HEIGHT)
        .loop_mode(LoopMode::loop_once())
        .run()
}


fn view(app: &App, frame: Frame) {
    // get canvas
    let draw = app.draw();
    // translate coordinate system from pixels to SIZE pixels per unit
    let gdraw = draw.scale(SIZE as f32)
                    // flip coordinate system around the y axis (0 top, + down)
                    .scale_y(-1.0)
                    // move center to top left corner and to center of that unit 
                    .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    gdraw.rect()
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(LINE_WIDTH)
        .w_h(1.0, 1.0)
        .x_y(0.0, 0.0)
        .rotate(0.0)
        ;

    
    // schotter logic
    for y in 0..ROWS {
        for x in 0..COLS {
            let cdraw = gdraw.x_y(x as f32, y as f32);
            let factor = y as f32 / ROWS as f32;
            let x_offset = factor * random_range(-0.5, 0.5);
            let y_offset = factor * random_range(-0.5, 0.5);
            let rotation = factor * random_range(-PI / 4.0, PI / 4.0); // -45 degree and 45 degree

            cdraw.rect()
                .no_fill()
                .stroke(BLACK)
                .stroke_weight(LINE_WIDTH)
                .w_h(1.0, 1.0)
                .x_y(x_offset, y_offset)
                .rotate(rotation)
                ;
        }
    }


    draw.background().color(SNOW);
    
    // render
    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
