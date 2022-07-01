use dzahui::{DzahuiWindow, Mesh, Dimension, Camera};
use glutin::{event_loop::{ControlFlow},event::{Event, WindowEvent}};
use glutin::event_loop::EventLoop;
use cgmath::{Matrix4, Vector3, Deg};
use std::{fs::File};
use gl;

fn main() {
    // Creating EventLoop
    let event_loop = EventLoop::new();

    // Creating DzahuiWindow
    let window = DzahuiWindow::new(600,800,(3,3), &event_loop,
    "/home/Arthur/Tesis/Dzahui/assets/vertex_shader.vs","/home/Arthur/Tesis/Dzahui/assets/fragment_shader.fs");

    // Creation of Mesh and setup
    let mesh_file = File::open("/home/Arthur/Tesis/Dzahui/assets/big_mesh.obj").unwrap();
    let mut mesh = Mesh::new(mesh_file,Dimension::D2);
    // Mesh setup. Can only be done once window object has been created. Find a way to relate the two.
    mesh.setup();

    // Creation of camera. Soon to be not static
    let camera = Camera::new(&mesh, 600.0, 800.0);
    println!("{:?}",camera.view_matrix);
    println!("{:?}",camera.projection_matrix);
    println!("{:?}",camera.near);
    window.shader.set_mat4("view", &camera.view_matrix);
    window.shader.set_mat4("projection", &camera.projection_matrix);

    event_loop.run(move |event, _, control_flow| {

        match event {
            Event::LoopDestroyed => (), // subscribing to events occurs here
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::Resized(physical_size) => window.context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {input, is_synthetic, ..} => {
                    if !is_synthetic && input.scancode == 1 {
                        // Close on esc
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => ()
            },
            Event::RedrawRequested(_) => (),
            _ => ()
        }
        // Render
        unsafe {
            // Update to some color
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            // Clear Screem
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // Draw triangles via ebo (indices)
            mesh.draw();
        }
        // Need to change old and new buffer to redraw
        window.context.swap_buffers().unwrap();
    })
}