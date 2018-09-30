use imlib2::{self, Image as Image2};
use xlib::{Display, Visual, Window};

use errors::ScreenshotError;
use Options;
use Rectangle;
use Region;

pub struct GUI {
    pub(crate) display: Display,
}

impl GUI {
    pub fn new() -> Result<Self, ScreenshotError> {
        let display = Display::connect(":0")?;
        Ok(GUI { display })
    }

    /// Captures the window and produces an Image.
    pub fn capture_window(&self, opt: &Options, window: Window) -> Result<Image2, ScreenshotError> {
        let attr = window.get_attributes()?;
        let mut width = attr.get_width();
        let mut height = attr.get_height();
        let root = attr.get_root();
        let (mut x, mut y, _) = self.display.translate_coordinates(window, 0, 0, root)?;
        println!("{} {}", window.as_raw(), root.as_raw());

        imlib2::context_set_display(&self.display);
        let visual = Visual::default(&self.display, 0);
        imlib2::context_set_visual(&visual);

        match opt.region {
            Region::Selection => {
                let capture = Image2::create_from_drawable(
                    window,
                    0,
                    x,
                    y,
                    width as i32,
                    height as i32,
                    true,
                )?;
                let region = self.interactive_select(capture)?;
                x = region.x;
                y = region.y;
                width = region.width;
                height = region.height;
            }
            _ => (),
        }

        Image2::create_from_drawable(window, 0, x, y, width as i32, height as i32, true)
            .map_err(|err| err.into())
    }

    /// Get the active window.
    pub fn get_active_window(&self) -> Result<Window, ScreenshotError> {
        self.display
            .get_input_focus()
            .map(|(window, _)| window)
            .map_err(|err| err.into())
    }

    /// Brings up an interactive selection GUI.
    pub fn interactive_select(&self, capture: Image2) -> Result<Rectangle, ScreenshotError> {
        // let window = SelectWindow::new(&self.display);
        // let root = self.display.get_default_root_window()?;

        // let root_im = root.get_image();

        // let mut done = 0;
        // let mut button_press = false;
        // while done == 0 && self.display.pending()? > 0 {
        //     let ev = self.display.next_event()?;
        //     match ev.kind() {
        //         EventKind::ButtonPress => {
        //             button_press = true;
        //         }
        //         EventKind::ButtonRelease => {
        //             if button_press {
        //                 done = 1;
        //             }
        //             button_press = false;
        //         }
        //         _ => (),
        //     }
        // }

        use gl;
        use glutin::{
            self,
            dpi::LogicalSize,
            os::unix::{WindowBuilderExt, WindowExt, XWindowType},
            ElementState, Event, EventsLoop, GlContext, GlWindow, KeyboardInput, MouseButton,
            MouseCursor, VirtualKeyCode, WindowBuilder, WindowEvent,
        };
        use nanovg::{self, Image, ImagePattern, PathOptions, StrokeOptions};
        use std::{f32::consts, mem, slice};

        // let attr = window.get_attributes()?;
        // let width = attr.get_width();
        // let height = attr.get_height();
        // let root = attr.get_root();
        // let (x, y, _) = self.display.translate_coordinates(window, 0, 0, root)?;
        let width = capture.get_width();
        let height = capture.get_height();

        let mut evl = EventsLoop::new();
        let mon = evl.get_primary_monitor();

        // TODO: handle error
        let wb = WindowBuilder::new()
            .with_x11_window_type(XWindowType::Splash)
            .with_decorations(false)
            .with_always_on_top(true)
            .with_dimensions(LogicalSize::new(width.into(), height.into()))
            .with_fullscreen(Some(mon));
        let ctx = glutin::ContextBuilder::new()
            .with_vsync(false)
            .with_multisampling(4)
            .with_double_buffer(Some(true))
            .with_srgb(true);
        let win = GlWindow::new(wb, ctx, &evl).expect("couldn't make window");
        win.set_position((0.0, 0.0).into());
        let f = win.get_hidpi_factor() as f64;

        // crosshair
        win.set_cursor(MouseCursor::Crosshair);
        // win.set_inner_size((width, height).into());
        // println!("size={:?} pos={:?} outer={:?}", win.get_inner_size(), win.get_inner_position(), win.get_outer_size());
        // println!("{:?}", win.get_hidpi_factor());

        let x = Display::from_handle(win.get_xlib_display().unwrap() as u64);
        let len;
        let raw_data;
        {
            let _g = x.grab();
            println!("grabbed");
            // let img = Image2::create_from_drawable(window, 0, 0, 0, width as i32, height as i32, true)?;
            imlib2::context_set_image(&capture);
            println!("set contexted");
            len = (width * height) as usize;
            // println!("{}", window.as_raw());
            raw_data = unsafe { slice::from_raw_parts(imlib2::image_get_data(), len) };
            println!("captured");

            unsafe {
                win.make_current().expect("couldn't make window");
                gl::load_with(|sym| win.get_proc_address(sym) as *const _);
            }
            println!("maked");
        }
        mem::forget(x);

        // convert ARGB to RGBA
        let mut data = vec![0u32; raw_data.len()];
        data.copy_from_slice(raw_data);
        for i in &mut data {
            // fix the colors
            *i = (*i & 0xff00ff00) | ((*i & 0xff) << 16) | ((*i >> 16) & 0xff);
        }

        // invert image
        let mut inverted = vec![0u32; raw_data.len()];
        inverted.copy_from_slice(raw_data);
        for i in &mut inverted {
            // fix the colors
            *i = (*i & 0xff000000) | !(*i & 0xffffff);
        }

        let ctx = nanovg::ContextBuilder::new()
            .build()
            .expect("couldn't init nanovg");

        let image = Image::new(&ctx)
            .build_from_rgba(width as usize, height as usize, data.as_slice())
            .expect("couldn't create image");

        let inverted_image = Image::new(&ctx)
            .build_from_rgba(width as usize, height as usize, inverted.as_slice())
            .expect("couldn't create image");

        let mut running = true;
        let mut down = false;
        // drag start
        let mut dx = -1.0f64;
        let mut dy = -1.0f64;
        // curr mouse
        let mut mx = -1.0f64;
        let mut my = -1.0f64;
        // rect
        let mut rectw = 0.0f64;
        let mut recth = 0.0f64;
        let mut delayed_down = false;
        let mut redraw = true;
        while running {
            if redraw {
                // let size = win.get_inner_size().unwrap();
                // let (width, height) = (size.width as i32, size.height as i32);

                unsafe {
                    gl::Viewport(0, 0, width as i32, height as i32);
                    gl::ClearColor(0.3, 0.3, 0.32, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
                }

                let (width, height) = (width as f64, height as f64);
                ctx.frame((width as f32, height as f32), f as f32, |frame| {
                    let path_opts = PathOptions::default();
                    frame.path(
                        |path| {
                            path.rect((0.0, 0.0), ((width * f) as f32, (height * f) as f32));
                            // path.fill(Color::from_rgba(200, 200, 200, 255), Default::default());
                            path.fill(
                                ImagePattern {
                                    image: &image,
                                    origin: (0.0, 0.0),
                                    size: (width as f32, height as f32),
                                    angle: 0.0 / 180.0 * consts::PI,
                                    alpha: 1.0,
                                },
                                Default::default(),
                            )
                        },
                        path_opts,
                    );
                    if down && rectw.abs() > 0.0 && recth.abs() > 0.0 {
                        frame.path(
                            |path| {
                                path.rect(
                                    ((dx * f) as f32, (dy * f) as f32),
                                    ((rectw * f) as f32, (recth * f) as f32),
                                );
                                path.stroke(
                                    // Color::from_rgba(0, 0, 0, 255),
                                    ImagePattern {
                                        image: &inverted_image,
                                        origin: (0.0, 0.0),
                                        size: (width as f32, height as f32),
                                        angle: 0.0 / 180.0 * consts::PI,
                                        alpha: 1.0,
                                    },
                                    StrokeOptions {
                                        width: 1.0,
                                        ..Default::default()
                                    },
                                );
                            },
                            path_opts,
                        );
                    }
                });
            }

            evl.poll_events(|event| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => running = false,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                state,
                                ..
                            },
                        ..
                    } => match (virtual_keycode, state) {
                        (Some(VirtualKeyCode::Escape), ElementState::Released) => {
                            if down {
                                down = false;
                            } else {
                                running = false;
                            }
                        }
                        _ => (),
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        mx = position.x;
                        my = position.y;
                        if down {
                            if delayed_down {
                                dx = mx;
                                dy = my;
                                delayed_down = false;
                            } else {
                                redraw = true;
                            }
                            rectw = mx - dx;
                            recth = my - dy;
                        }
                    }
                    WindowEvent::MouseInput { button, state, .. } => match button {
                        MouseButton::Left => {
                            down = match state {
                                ElementState::Pressed => {
                                    delayed_down = true;
                                    if mx < 0.0 || my < 0.0 {
                                    } else {
                                        dx = mx;
                                        dy = my;
                                    }
                                    true
                                }
                                ElementState::Released => {
                                    if down && rectw.abs() > 0.0 && recth.abs() > 0.0 {
                                        running = false;
                                    }
                                    false
                                }
                            };
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            });

            win.swap_buffers().expect("couldn't swap buffers");
        }
        if rectw.abs() > 0.0 && recth.abs() > 0.0 {
            let mut x = dx;
            let mut y = dy;
            if rectw < 0.0 {
                x += rectw;
            }
            if recth < 0.0 {
                y += recth;
            }
            Ok(Rectangle::new(
                (x * f) as i32,
                (y * f) as i32,
                (rectw.abs() * f) as u32,
                (recth.abs() * f) as u32,
            ))
        } else {
            Err(ScreenshotError::Error)
        }
    }
}
