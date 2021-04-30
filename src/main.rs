extern crate gl;
extern crate sdl2;
extern crate stb_image;
extern crate image;
extern crate serde_json;
extern crate glm;
extern crate nalgebra_glm;
extern crate stopwatch;
pub mod render_gl;
pub mod world;
use std::ffi::CString;
use std::io::{stdout, Write};

fn main() {
    //Settings
    //Current amount of textures
    const AMOUNT_TEXTURES: usize =  4;
    const SQUARE_CHUNK_WIDTH: u32 = 16;//16;
    const BLOCK_RADIUS: f32 = 0.3; 
    const CHUNKS_LAYERS_FROM_PLAYER: u32 = 2; //Actualy its the width of the rendered area in chunks // HAve it as odd number
    const WINDOW_WIDTH: u32 = 1500;
    const WINDOW_HEIGHT: u32 = 1000;
    const VIEW_DISTANCE: f32 = 50.0;
    let mut mesh = false;

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    
    let window = video_subsystem
        .window("MinecraftRS", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    
    
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    // let window_init = init_window(WINDOW_WIDTH.clone(), WINDOW_HEIGHT.clone());
    // let window = window_init.0;
    // let sdl = window_init.1;
    //Camera
    let mut camera_pos = glm::vec3(0.0, 0.0, 0.0);
    let mut camera_front = glm::vec3(0.0, 0.0, -1.0);
    let camera_up = glm::vec3(0.0, 1.0, 0.0);

    let mut yaw = -90.0;
    let mut pitch = 0.0;
    let mut fov = 85.0;

    //Mouse state
    let mut first_mouse = true;
    let mut last_x = 800.0 / 2.0;
    let mut last_y = 600.0 / 2.0;


    //Timing
    let mut delta_time = 0.0;
    let mut last_frame = 0.0;

    //Set mouse to be bound in the window and infinite movement
    sdl.mouse().capture(true);
    sdl.mouse().set_relative_mouse_mode(true);

    // set up shader program
    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();    

    let vertices: Vec<f32> = vec![
    // Back face
   -0.15, -0.15, -0.15,  1.0, 1.0, // Bottom-left
    0.15,  0.15, -0.15,  0.0, 0.0, // top-right
    0.15, -0.15, -0.15,  0.0, 1.0, // bottom-right         
    0.15,  0.15, -0.15,  0.0, 0.0, // top-right
   -0.15, -0.15, -0.15,  1.0, 1.0, // bottom-left
   -0.15,  0.15, -0.15,  1.0, 0.0, // top-left
   // Front face
   -0.15, -0.15,  0.15,  1.0, 1.0, // bottom-left
    0.15, -0.15,  0.15,  0.0, 1.0, // bottom-right
    0.15,  0.15,  0.15,  0.0, 0.0, // top-right
    0.15,  0.15,  0.15,  0.0, 0.0, // top-right
   -0.15,  0.15,  0.15,  1.0, 0.0, // top-left
   -0.15, -0.15,  0.15,  1.0, 1.0, // bottom-left
   // Left face
   -0.15,  0.15,  0.15,  1.0, 0.0, // top-right
   -0.15,  0.15, -0.15,  0.0, 0.0, // top-left
   -0.15, -0.15, -0.15,  0.0, 1.0, // bottom-left
   -0.15, -0.15, -0.15,  0.0, 1.0, // bottom-left
   -0.15, -0.15,  0.15,  1.0, 1.0, // bottom-right
   -0.15,  0.15,  0.15,  1.0, 0.0, // top-right
   // Right face
    0.15,  0.15,  0.15,  0.0, 0.0, // top-left
    0.15, -0.15, -0.15,  1.0, 1.0, // bottom-right
    0.15,  0.15, -0.15,  1.0, 0.0, // top-right         
    0.15, -0.15, -0.15,  1.0, 1.0, // bottom-right
    0.15,  0.15,  0.15,  0.0, 0.0, // top-left
    0.15, -0.15,  0.15,  0.0, 1.0, // bottom-left     
   // Bottom face
   -0.15, -0.15, -0.15,  1.0, 0.0, // top-right
    0.15, -0.15, -0.15,  0.0, 0.0, // top-left
    0.15, -0.15,  0.15,  0.0, 1.0, // bottom-left
    0.15, -0.15,  0.15,  1.0, 0.0, // bottom-left
   -0.15, -0.15,  0.15,  0.0, 0.0, // bottom-right
   -0.15, -0.15, -0.15,  0.0, 1.0, // top-right
   // Top face
   -0.15,  0.15, -0.15,  0.0, 1.0, // top-left
    0.15,  0.15,  0.15,  1.0, 0.0, // bottom-right
    0.15,  0.15, -0.15,  1.0, 1.0, // top-right     
    0.15,  0.15,  0.15,  1.0, 0.0, // bottom-right
   -0.15,  0.15, -0.15,  0.0, 1.0, // top-left
   -0.15,  0.15,  0.15,  0.0, 0.0  // bottom-left 

    ];

    //Y is height
    //X is sideways in the x axis
    //Z is sideways in the what would normaly be y axis
    
    // set up vertex array object
    let mut vao: gl::types::GLuint = 0;
    let mut vbo: gl::types::GLuint = 0;
    let mut ebo: gl::types::GLuint = 0;
    unsafe {

        gl::GenBuffers(1, &mut vbo);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData( gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut ebo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer( 0,3, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as gl::types::GLint, std::ptr::null(),);

        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as gl::types::GLint, (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        gl::ClearColor(0.49, 0.87, 0.96, 1.0); // Divide smth like 120 by 255 and you get the color you want. Replace 120 with what you have in rgb
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
    }

    
    let mut time_increment: f32 = 0.0;
    let mut stopwatch = stopwatch::Stopwatch::new();
    stopwatch::Stopwatch::start(&mut stopwatch);
    let mut world: world::World = world::World::new(&AMOUNT_TEXTURES, &(camera_pos / BLOCK_RADIUS), &SQUARE_CHUNK_WIDTH, &BLOCK_RADIUS, &shader_program, &CHUNKS_LAYERS_FROM_PLAYER, &VIEW_DISTANCE);
    print!("   TimeElapsed: {}",stopwatch::Stopwatch::elapsed_ms(&stopwatch));
    stopwatch::Stopwatch::reset(&mut stopwatch);
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main, 
                
                sdl2::event::Event::KeyDown { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                    let camera_speed = 2.5 * delta_time;
                    

                    //Change to polygon mesh mode
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Q {
                        unsafe {
                            if !mesh {
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                                mesh = true;
                            } else{
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                                mesh = false;
                            }
                        }
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Escape {
                        break 'main;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Space {
                        camera_pos = camera_pos + glm::vec3(0.0, camera_speed, 0.0);
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::LCtrl {
                        camera_pos = camera_pos - glm::vec3(0.0, camera_speed, 0.0);
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::W {
                        camera_pos = camera_pos + glm::vec3(camera_speed * camera_front.x, camera_speed * camera_front.y, camera_speed * camera_front.z);
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::S {
                        camera_pos = camera_pos - glm::vec3(camera_speed * camera_front.x, camera_speed * camera_front.y, camera_speed * camera_front.z);
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::A {
                        camera_pos = camera_pos - glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed;
                    }
                    if scancode.unwrap() == sdl2::keyboard::Scancode::D {
                        camera_pos = camera_pos + glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed;
                    }
                    
                    
                },
                
                sdl2::event::Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x, y, xrel: _, yrel: _ } => {
                    if first_mouse
                    {
                        last_x = x as f32;
                        last_y = y as f32;
                        first_mouse = false;
                    }

                    let mut xoffset = x as f32 - last_x;
                    let mut yoffset = last_y - y as f32; // reversed since y-coordinates go from bottom to top
                    last_x = x as f32;
                    last_y = y as f32;

                    let sensitivity = 0.1; // change this value to your liking
                    xoffset *= sensitivity;
                    yoffset *= sensitivity;

                    yaw += xoffset;
                    pitch += yoffset;

                    // make sure that when pitch is out of bounds, screen doesn't get flipped
                    if pitch > 89.0 {
                        pitch = 89.0;
                    }
                    if pitch < -89.0 {
                        pitch = -89.0;
                    }

                    let mut front = glm::vec3(0.0, 0.0, 0.0);
                    front.x = glm::cos(glm::radians(yaw)) * glm::cos(glm::radians(pitch));
                    front.y = glm::sin(glm::radians(pitch));
                    front.z = glm::sin(glm::radians(yaw)) * glm::cos(glm::radians(pitch));
                    camera_front = glm::normalize(front);
                },
                sdl2::event::Event::MouseWheel { timestamp: _, window_id: _, which: _, x: _, y, direction: _ } => {
                    if fov >= 1.0 && fov <= 90.0 {
                        fov -= y as f32;
                    }  
                    if  fov < 1.0 {
                        fov = 1.0;
                    }   
                    if  fov > 90.0 {
                        fov = 90.0;
                    }
                },
                _ => {}
            }
        }     
        



        unsafe {

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 
            shader_program.set_used();

            let current_frame = time_increment as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            let projection = glm::ext::perspective(glm::radians(fov), (WINDOW_WIDTH as f32)/(WINDOW_HEIGHT as f32), 0.1, 100.0);
            let projection_loc = gl::GetUniformLocation(shader_program.id(), "projection".as_ptr() as *const std::os::raw::c_char);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, &projection[0][0]);

            let view = glm::ext::look_at(camera_pos, camera_pos + camera_front, camera_up);
            let view_loc = gl::GetUniformLocation(shader_program.id(), "view".as_ptr() as *const std::os::raw::c_char);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

            gl::BindVertexArray(vao);

            world::World::render(&mut world, &(camera_pos / BLOCK_RADIUS), &vao);

            gl::BindVertexArray(0);

        }
        time_increment += 0.02;
        window.gl_swap_window();
        let mut x_axis = f32::abs(camera_front.x);
        let mut y_axis = f32::abs(camera_front.y);
        let mut z_axis = f32::abs(camera_front.z);
        let x_sign = if camera_front.x > 0.0 {"+"} else {"-"};
        let y_sign = if camera_front.y > 0.0 {"+"} else {"-"};
        let z_sign = if camera_front.z > 0.0 {"+"} else {"-"};

        if(x_axis > y_axis && x_axis > z_axis){
            println!("Axis: {}X",x_sign);
        }else if(y_axis > x_axis && y_axis > z_axis){
            println!("Axis: {}Y",y_sign);
        }else if(z_axis > y_axis && z_axis > x_axis){
            println!("Axis: {}Z",z_sign);
        }
        // stopwatch::Stopwatch::stop(&mut stopwatch);
        // print!(" TimeElapsed: {}",stopwatch::Stopwatch::elapsed_ms(&stopwatch));
        // stopwatch::Stopwatch::reset(&mut stopwatch);
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        //println!("");
        //Previous = 10
        //break;
    }
}
