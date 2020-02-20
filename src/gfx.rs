extern crate gl;
extern crate image;

use image::ColorType;
use image::GenericImage;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null_mut;

use gl::types::*;

use crate::utils;
use std::ptr::null;

/// Flags stand for types.
type Flag = GLuint;
type Status = GLint;
type Shader = GLuint;
type Program = GLuint;
type Texture = GLuint;
#[warn(dead_code)]
pub const TEXTURE0: u32 = gl::TEXTURE0;
#[warn(dead_code)]
pub const TEXTURE1: u32 = gl::TEXTURE1;
#[warn(dead_code)]
pub const TEXTURE2: u32 = gl::TEXTURE2;
#[warn(dead_code)]
pub const TEXTURE3: u32 = gl::TEXTURE3;

/// Why not just a GLuint ?
/// We have to remember the size
pub struct VertexArray {
    vertex_array: GLuint,
    length: i32,
}

/// Clears screen with a color.
pub fn clear() {
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

/// Prints information about graphics API, gpu and driver version.
pub fn info() {
    let version = string(gl::VERSION);
    let vendor = string(gl::VENDOR);
    let renderer = string(gl::RENDERER);

    println!("{}\n{}\n{}", version, vendor, renderer)
}

/// Gets OpenGL string and converts it to rust string.
pub fn string(flag: Flag) -> String {
    let str = unsafe {
        let data = CStr::from_ptr(gl::GetString(flag) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };
    return str;
}

/// Reads, compiles and links shaders into a program.
pub fn create_program(vertex_shader: &str, fragment_shader: &str) -> Program {
    // Work with vertex shader
    // Get content
    let vertex_content = utils::read_all(vertex_shader);
    let fragment_content = utils::read_all(fragment_shader);
    // Compile it
    let vertex_shader = unsafe { compile_shader(gl::VERTEX_SHADER, vertex_content.as_str()) };
    let fragment_shader = unsafe { compile_shader(gl::FRAGMENT_SHADER, fragment_content.as_str()) };
    // Creae the program and attach shaders
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        // Delete shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    return program;
}

unsafe fn compile_shader(shader_type: Flag, source: &str) -> Shader {
    // Create shader
    let shader: Shader = gl::CreateShader(shader_type);
    // Convert to C string
    let source =
        CStr::from_bytes_with_nul(source.as_bytes()).expect("Unable to properly read shader.");
    // .expect("Couldn't read properly given shader source code.");
    // Compile shader
    gl::ShaderSource(shader, 1, &source.as_ptr(), null());
    gl::CompileShader(shader);
    // Any error ?
    let mut status: i32 = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    // Something went wrong
    if status == 0 {
        // Get length of the log
        let mut len: Status = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        // Allocate buffer
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(len as usize));
        let error: CString = CString::from_vec_unchecked(buffer);
        gl::GetShaderInfoLog(shader, len, null_mut(), error.as_ptr() as *mut GLchar);
        panic!(error.to_string_lossy().into_owned());
    }
    return shader;
}

/// Binds program.
pub fn use_program(program: Program) {
    unsafe {
        gl::UseProgram(program);
    }
}

/// Generates one VertexArray.
pub fn gen_vertex_array(length: i32) -> VertexArray {
    let mut vao: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    return VertexArray {
        vertex_array: vao,
        length: length,
    };
}

/// Generates one ArrayBuffer
pub fn gen_buffer(vertex_array: &VertexArray, data: Vec<f32>, size: i32, attrib_index: u32) {
    let mut buffer: GLuint = 0;
    unsafe {
        let length = (data.len() * size_of::<f32>()) as isize;
        gl::BindVertexArray(vertex_array.vertex_array);
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            length,
            data.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(attrib_index);
        gl::VertexAttribPointer(attrib_index, size, gl::FLOAT, 0, 0, null());
    }
}

/// Draws a vertex array
pub fn draw_arrays(vertex_array: &VertexArray) {
    unsafe {
        gl::BindVertexArray(vertex_array.vertex_array);
        gl::DrawArrays(gl::TRIANGLES, 0, vertex_array.length);
    }
}

/// Generate a texture.
/// Empty texture for the moment.
pub fn gen_empty_texture(width: i32, height: i32, empty: bool) -> Texture {
    let mut texture: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        if !empty {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                null(),
            );
        }
    }
    return texture;
}

/// Feed an image with data.
pub fn tex_image_2d(image_path: &str) -> Texture {
    let data = image::open(image_path).expect(&format!("Unable to open image {}.", image_path));
    let mut flag: GLenum = 0;
    // Get pixel format
    if data.color() == ColorType::RGBA(8) {
        flag = gl::RGBA;
    } else if data.color() == ColorType::RGB(8) {
        flag = gl::RGB;
    } else {
        panic!(format!("Unsupported pixel format {}.", image_path))
    }
    let rgb = data.raw_pixels();
    let width = data.width() as i32;
    let height = data.height() as i32;
    // Get texture
    println!("SALUT !!!");
    let texture = gen_empty_texture(width, height, true);
    unsafe {
        println!("SALUT !!!");
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            flag as i32,
            width,
            height,
            0,
            flag,
            gl::UNSIGNED_BYTE,
            &rgb[0] as *const u8 as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    return texture;
}

pub fn bind_texture(texture: Texture, slot: u32) {
    unsafe {
        gl::ActiveTexture(slot);
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }
}
