use glam::*;
use std::ffi::CString;
use sdl2::controller::*;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use std::fs;

mod audio_manager;
mod bitmap_string;
mod block;
mod bomb;
mod configuration;
mod common;
mod constants;
mod game;
mod gui;
mod laser;
mod level;
mod monster;
mod ogl;
mod star;
mod random;
mod renderers;
mod screens;
mod shader_manager;
mod ship;
mod ufo;

use constants::*;
use game::{Game, PlayerCommand};
use ogl::buffer::UniformBuffer;
use configuration::Configuration;

use crate::audio_manager::{AudioManager, SoundType};
use crate::common::DisplayInfo;
use crate::shader_manager::{ShaderManager, ShaderType};


//write configuration to the file
fn write_config(config: &Configuration) {
    let s = config.to_string();

    if let Err(e) = fs::write("config.toml", s) {
        println!("Can't write to \"config.toml\". {e}");
    }
}

fn main() {

    //read configuration file or generate a default one
    let config = match Configuration::from_file("config.toml") {
        Ok(c) => c,
        Err(e) => {
            println!("Can't parse \"config.toml\" file.\n{e}\nGenerating default configuration.");
            Configuration::default()
        },
    };

    //make a copy of configuration to see a changes
    let old_config = config.clone();

    //init SDL2 context
    let sdl_ctx = sdl2::init().unwrap();
    let sdl_timer = sdl_ctx.timer().unwrap();
    let sdl_video = sdl_ctx.video().unwrap();
    let sdl_game_controller = sdl_ctx.game_controller().unwrap();
    let mut game_controller = sdl_game_controller.open(0).ok();

    //init audio system
    let mut audio_manager = AudioManager::new(config.volume, AUDIO_CHANNELS);
    audio_manager.load_sound("assets/ship_laser.wav", SoundType::ShipFire);
    audio_manager.load_sound("assets/ship_hit.wav", SoundType::ShipHit);
    audio_manager.load_sound("assets/monster_laser.wav", SoundType::MonsterFire);
    audio_manager.load_sound("assets/monster_hit.wav", SoundType::MonsterHit);
    audio_manager.load_sound("assets/ufo_hit.wav", SoundType::UfoHit);
    audio_manager.load_sound("assets/bomb_hit.wav", SoundType::BombHit);

    //list all aviable displays and their modes
    println!("\nAll aviable displays and their modes. Please, if you want to change the default settings, do it in 'config.toml' file.");
    let displays = sdl_video.num_video_displays().unwrap();
    for display_idx in 0..displays {
        let display_name = sdl_video.display_name(display_idx).unwrap();
        println!("\ndisplay modes for display '{display_name}' with index {display_idx}:");

        let modes = sdl_video.num_display_modes(display_idx).unwrap();
        for i in 0..modes {
            let dm = sdl_video.display_mode(display_idx, i).unwrap();
            println!("mode {i} - {}x{} @{}Hz", dm.w, dm.h, dm.refresh_rate);
        }
    }

    //get display properties from selected display
    let dm = match sdl_video.display_mode(config.display_index, config.display_mode) {
        Ok(dm) => dm,
        Err(s) => { println!("Display error '{s}'"); return; },
    };

    //create sdl window
    let title = format!("Nebulon Monsters v{}", env!("CARGO_PKG_VERSION"));
    let display_bounds = sdl_video.display_bounds(config.display_index).unwrap();
    let window = sdl_video.window(&title, dm.w as u32, dm.h as u32)
        .opengl()
        .position(display_bounds.x, display_bounds.y)
        .fullscreen_desktop()
        // .fullscreen()
        .build()
        .unwrap();

    //create openGL context and load opengl functions
    let _gl_ctx = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    //print opengl version
    let version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _) };
    println!("OpenGL version: {:?}", version);

    //enable vsync
    sdl_video.gl_set_swap_interval(if config.vsync { 1 } else { 0 }).unwrap();

    let da = window.size();
    let display_info = DisplayInfo::new(dm.w, dm.h, da.0 as i32, da.1 as i32);
    unsafe {
        gl::Viewport(0, 0, da.0 as i32, da.1 as i32);
        gl::ClearColor(BACKGROUND_COLOR.x, BACKGROUND_COLOR.y, BACKGROUND_COLOR.z, 1.0);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);
    }

    //load and compile shaders
    let mut shader_manager = ShaderManager::new();
    shader_manager.load_shader_from_cstr(&CString::new(include_str!("shaders/bitmap_glyph_v.glsl")).unwrap(), &CString::new(include_str!("shaders/bitmap_glyph_f.glsl")).unwrap(), ShaderType::BitmapStringShader);
    shader_manager.load_shader_from_cstr(&CString::new(include_str!("shaders/frame_v.glsl")).unwrap(), &CString::new(include_str!("shaders/frame_f.glsl")).unwrap(), ShaderType::FrameShader);
    shader_manager.load_shader_from_cstr(&CString::new(include_str!("shaders/lives_v.glsl")).unwrap(), &CString::new(include_str!("shaders/lives_f.glsl")).unwrap(), ShaderType::LivesShader);
    shader_manager.load_shader_from_cstr(&CString::new(include_str!("shaders/monster_v.glsl")).unwrap(), &CString::new(include_str!("shaders/monster_f.glsl")).unwrap(), ShaderType::MonsterShader);
    shader_manager.load_shader_from_cstr(&CString::new(include_str!("shaders/simple_v.glsl")).unwrap(), &CString::new(include_str!("shaders/simple_f.glsl")).unwrap(), ShaderType::SimpleShader);
    shader_manager.load_shader_from_cstr(&CString::new(include_str!("shaders/ship_v.glsl")).unwrap(), &CString::new(include_str!("shaders/ship_f.glsl")).unwrap(), ShaderType::ShipShader);
    shader_manager.load_shader_from_cstr(&CString::new(include_str!("shaders/ufo_v.glsl")).unwrap(), &CString::new(include_str!("shaders/ufo_f.glsl")).unwrap(), ShaderType::UfoShader);

    //projection matrix is in "drawable area" size to compensate for desktop scaling factors
    let projection_matrix = Mat4::orthographic_rh_gl(0.0, dm.w as f32, dm.h as f32, 0.0, -1.0, 1.0);
   
    //create and bind uniform buffers for projection matrix
    let matrices = UniformBuffer::new();
    matrices.bind_buffer_base(constants::UNIFORM_BUFFER_BINDING_MATRICES);
    matrices.buffer_data(&[projection_matrix], gl::STATIC_DRAW);

    //game object, and player commands
    let mut game = Game::new(&display_info, config, &shader_manager);
    let mut game_commands = Vec::<PlayerCommand>::new();
    let mut single_command: Option<PlayerCommand>;

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    let mut old_time = sdl_timer.performance_counter();

    'main_loop: loop {

        //clear commands from the last iteration
        game_commands.clear();
        single_command = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => { break 'main_loop; },
                Event::KeyDown { keycode: Some(key_code), .. } => {
                    match key_code {
                        Keycode::Escape => single_command = Some(PlayerCommand::Escape),
                        Keycode::Return | Keycode::Space => single_command = Some(PlayerCommand::Select),
                        Keycode::Up | Keycode::Left => single_command = Some(PlayerCommand::Up),
                        Keycode::A | Keycode::W => single_command = Some(PlayerCommand::Up),
                        Keycode::Down | Keycode::Right => single_command = Some(PlayerCommand::Down),
                        Keycode::S | Keycode::D => single_command = Some(PlayerCommand::Down),
                        _ => (),
                    }
                },
                Event::ControllerButtonDown { button, .. } => {
                    match button {
                        Button::Back => single_command = Some(PlayerCommand::Escape),
                        Button::Start | Button::A | Button::B | Button::X | Button::Y => single_command = Some(PlayerCommand::Select),
                        Button::DPadUp | Button::DPadLeft => single_command = Some(PlayerCommand::Up),
                        Button::DPadDown | Button::DPadRight => single_command = Some(PlayerCommand::Down),
                        _ => (),
                    }
                },
                Event::MouseMotion { x, y, .. } => {
                    single_command = Some(PlayerCommand::MouseMove(x, y));
                },
                Event::MouseButtonUp { x, y, .. } => {
                    single_command = Some(PlayerCommand::Click(x, y));
                },
                Event::ControllerDeviceAdded { which: id, .. } => {
                    println!("device with id \"{}\" and name \"{}\" detected", id, sdl_game_controller.name_for_index(id).unwrap());
                    game_controller = sdl_game_controller.open(id).ok();
                },
                Event::ControllerDeviceRemoved { which: id, .. } => {
                    println!("device with id \"{}\" removed", id);
                    game_controller = None;
                },
                _ => {}
            }
        }

        let new_time = sdl_timer.performance_counter();
        let delta_time = (new_time - old_time) as f32 / sdl_timer.performance_frequency() as f32;
        old_time = new_time;

        //players movement keys should be evaluated each frame, not only when event arrives
        for key in event_pump.keyboard_state().pressed_scancodes() {
            match key {
                Scancode::Right | Scancode::D => game_commands.push(PlayerCommand::Right),
                Scancode::Left | Scancode::A => game_commands.push(PlayerCommand::Left),
                Scancode::Space => game_commands.push(PlayerCommand::Fire),
                _ => (),
            }
        }

        //process game controller 
        if let Some(ref gc) = game_controller {
            if gc.button(Button::DPadLeft) {
                game_commands.push(PlayerCommand::Left);
            }

            if gc.button(Button::DPadRight) {
                game_commands.push(PlayerCommand::Right);
            }

            //what-ever button to fire
            if gc.button(Button::A) || gc.button(Button::B) || gc.button(Button::X) || gc.button(Button::Y) {
                game_commands.push(PlayerCommand::Fire);
            }
        }

        //when game update function returns true, it is signal to end the program
        if game.update(&game_commands, &single_command, delta_time, &audio_manager, &shader_manager) {
            break;
        }

        //render it
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        game.render(&shader_manager, delta_time);
        window.gl_swap_window();
    }

    //save configuration if changed
    if old_config != game.configuration() {
        write_config(&game.configuration());
    }
}
