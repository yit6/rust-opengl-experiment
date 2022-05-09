mod teapot;
mod controls;
pub mod quaternion;
mod transform;

use std::ops::Mul;

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

#[derive(Copy, Clone)]
struct Color {
    rgb: [f32; 3],
}

implement_vertex!(Vertex, position);
implement_vertex!(Color, rgb);

//fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
//let f = {
//let f = direction;
//let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
//let len = len.sqrt();
//[f[0] / len, f[1] / len, f[2] / len]
//};
//
//let s = [up[1] * f[2] - up[2] * f[1],
//up[2] * f[0] - up[0] * f[2],
//up[0] * f[1] - up[1] * f[0]];
//
//let s_norm = {
//let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
//let len = len.sqrt();
//[s[0] / len, s[1] / len, s[2] / len]
//};
//
//let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
//f[2] * s_norm[0] - f[0] * s_norm[2],
//f[0] * s_norm[1] - f[1] * s_norm[0]];
//
//let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
//-position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
//-position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
//
//[
//[s_norm[0], u[0], f[0], 0.0],
//[s_norm[1], u[1], f[1], 0.0],
//[s_norm[2], u[2], f[2], 0.0],
//[p[0], p[1], p[2], 1.0],
//]
//}

fn main() {
    use glium::{glutin, Surface};

    let mut last_time = std::time::Instant::now();

    let mut camera_transform = transform::Transform::new();
    camera_transform.translate([0.0,0.0,2.0f32]);

    let mut controls = controls::Controls::new();

    let mut teapot_transform = transform::Transform::new();
    teapot_transform.scale(0.01);
    teapot_transform.set_translation([0.0, 0.0, 2.0f32]);

    let vertex_shader_src = include_str!("vert.glsl"); 
    let fragment_shader_src = include_str!("frag.glsl");

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;

    event_loop.run(move |ev, _, control_flow| {

        let current_time = std::time::Instant::now();

        let delta_t = (current_time - last_time).subsec_nanos() as f32 / 1000000.0;  

        last_time = current_time;

        let next_frame_time = current_time + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                    glutin::event::ElementState::Pressed => match input.virtual_keycode {
                        Some(glutin::event::VirtualKeyCode::W    ) => { controls.w     = true },
                        Some(glutin::event::VirtualKeyCode::S    ) => { controls.s     = true },
                        Some(glutin::event::VirtualKeyCode::A    ) => { controls.a     = true },
                        Some(glutin::event::VirtualKeyCode::D    ) => { controls.d     = true },
                        Some(glutin::event::VirtualKeyCode::Up   ) => { controls.up    = true },
                        Some(glutin::event::VirtualKeyCode::Down ) => { controls.down  = true },
                        Some(glutin::event::VirtualKeyCode::Left ) => { controls.left  = true },
                        Some(glutin::event::VirtualKeyCode::Right) => { controls.right = true },
                        _ => (),
                    }
                    glutin::event::ElementState::Released => match input.virtual_keycode {
                        Some(glutin::event::VirtualKeyCode::W    ) => { controls.w     = false },
                        Some(glutin::event::VirtualKeyCode::S    ) => { controls.s     = false },
                        Some(glutin::event::VirtualKeyCode::A    ) => { controls.a     = false },
                        Some(glutin::event::VirtualKeyCode::D    ) => { controls.d     = false },
                        Some(glutin::event::VirtualKeyCode::Up   ) => { controls.up    = false },
                        Some(glutin::event::VirtualKeyCode::Down ) => { controls.down  = false },
                        Some(glutin::event::VirtualKeyCode::Left ) => { controls.left  = false },
                        Some(glutin::event::VirtualKeyCode::Right) => { controls.right = false },
                        _ => (),
                    }
                },
                _ => return,
            },
            _ => (),
        }

        t += 0.0002;
        let s: f32 = 0.05;

        //cam_q = cam_q * quaternion::from_axis_angle([0.0, 0.0, 1.0], delta_t/400.0);

        if controls.up    { camera_transform.rotate(quaternion::from_axis_angle([1.0, 0.0, 0.0],  delta_t/400.0)) }
        if controls.down  { camera_transform.rotate(quaternion::from_axis_angle([1.0, 0.0, 0.0], -delta_t/400.0)) }
        if controls.left  { camera_transform.rotate(quaternion::from_axis_angle([0.0, 1.0, 0.0],  delta_t/400.0)) }
        if controls.right { camera_transform.rotate(quaternion::from_axis_angle([0.0, 1.0, 0.0], -delta_t/400.0)) }

        if controls.w { camera_transform.local_translate([0.0, 0.0,  0.03 * delta_t]) }
        if controls.s { camera_transform.local_translate([0.0, 0.0, -0.03 * delta_t]) }
        if controls.a { camera_transform.local_translate([-0.03 * delta_t, 0.0, 0.0]) }
        if controls.d { camera_transform.local_translate([ 0.03 * delta_t, 0.0, 0.0]) }

        teapot_transform.rotate(quaternion::from_axis_angle([0.0, 1.0, 0.0], delta_t/100.0));

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        println!("{:?}", camera_transform);
        println!("{:?}", camera_transform.view_matrix());

        let uniforms = uniform! {
            perspective: perspective,
            view: camera_transform.view_matrix(),
            model: teapot_transform.to_matrix(),
            //model: [
                //[0.01 *  t.cos(), 0.0, 0.01 * t.sin(), 0.0],
                //[0.0, 0.01, 0.0, 0.0],
                //[0.01 * -t.sin(), 0.0, 0.01 * t.cos(), 0.0],
                //[0.0, 0.0, 2.0, 1.0f32],
            //],
            u_light : [-1.0, 0.4, 0.9f32]
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw((&positions, &normals), &indices, &program, &uniforms, &params).unwrap();

        target.finish().unwrap();
    });
}
