use glutin::{event_loop::{ControlFlow, EventLoop},event::{Event, WindowEvent, DeviceEvent, ElementState}};
use dzahui::{DzahuiWindow, Mesh2D, Mesh3D, Camera, Drawable, Binder};
use cgmath::{Point3};
use gl;

fn main() {
    // Creating EventLoop
    let event_loop = EventLoop::new();

    // Creating window with predetermined configuration
    let window = DzahuiWindow::new(600,800,(3,3), &event_loop,
    "/home/Arthur/Tesis/Dzahui/assets/vertex_shader.vs","/home/Arthur/Tesis/Dzahui/assets/fragment_shader.fs");

    // Creation of Mesh and setup
    let mesh = Mesh3D::new("/home/Arthur/Tesis/Dzahui/assets/cube.obj");

    // Creation of binding variables
    let mut binder = Binder::new(0,0,0);

    // translation for mesh to always be near (0,0)
    window.shader.set_mat4("model", &mesh.model_matrix);
    println!("{:?}",mesh.model_matrix);
    // Mesh setup. Can only be done once window object has been created. Find a way to relate the two.
    mesh.setup(&mut binder);

    // Creation of camera. Soon to be not static
    let mut camera = Camera::new(&mesh, 600.0, 800.0);
    println!("{:?}",camera.view_matrix);
    println!("{:?}",camera.projection_matrix);
    window.shader.set_mat4("view", &camera.view_matrix);
    window.shader.set_mat4("projection", &camera.projection_matrix);

    event_loop.run(move |event, _, control_flow| {

        match event {
            
            Event::LoopDestroyed => (), // subscribing to events occurs here
            Event::WindowEvent {event, ..} => match event {
                
                WindowEvent::Resized(physical_size) => window.context.resize(physical_size),
                
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                
                // Close on esc
                WindowEvent::KeyboardInput {input, is_synthetic, ..} => {
                    if !is_synthetic && input.scancode == 1 {
                        *control_flow = ControlFlow::Exit;
                    }
                },

                _ => ()
            },
            
            Event::DeviceEvent { device_id, event } => {
                match event {
                    
                    // to activate moving camera
                    DeviceEvent::Button { button, state } => {
                        match button {
                            2 => {
                                match state {
                                    ElementState::Pressed => {
                                        camera.active_view_change = true;
                                    },
                                    ElementState::Released => {
                                        camera.active_view_change = false;
                                    }
                                }
                            },
                            _ => {}
                        }
                    },

                    // to move camera
                    DeviceEvent::MouseMotion { delta: (x, y) } => {
                        if camera.active_view_change {
                            let x_offset = (x as f32) * camera.camera_sensitivity;
                            let y_offset = (y as f32) * camera.camera_sensitivity;
                            camera.theta += y_offset;
                            camera.phi += x_offset;
                            
                            // Do not allow 0 (or 180) degree angle (coincides with y-axis)
                            if camera.theta < 1.0 {
                                camera.theta = 1.0;
                            } else if camera.theta < -179.0 {
                                camera.theta = -179.0;
                            }
    
                            // update position
                            camera.camera_position = Point3::new(camera.theta.to_radians().sin()*camera.phi.to_radians().sin(),
                            camera.theta.to_radians().cos(),camera.theta.to_radians().sin()*camera.phi.to_radians().cos()) * camera.radius;
                            
                            // generate new matrix
                            camera.modify_view_matrix();
                        }


                    }

                    _ => {}
                }
            },

            Event::RedrawRequested(_) => (),

            _ => (),
        }
        // Render
        unsafe {
            // Update to some color
            gl::ClearColor(0.33, 0.33, 0.33, 0.8);
            // Clear Screem
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // send new view matrix
            window.shader.set_mat4("view", &camera.view_matrix);
            // Draw triangles via ebo (indices)
            mesh.draw();
        }
        // Need to change old and new buffer to redraw
        window.context.swap_buffers().unwrap();
    })
}