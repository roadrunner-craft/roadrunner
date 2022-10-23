use crate::ops::Bindable;
use crate::utils::Identifiable;

use gl::types::{GLint, GLsizei, GLuint};
use math::vector::Vector2;
use std::ptr;

#[derive(Default, Debug)]
pub struct FrameBuffer {
    id: GLuint,
    texture: GLuint,
    unit: GLuint,
    depth: Option<GLuint>,
    size: Vector2,
}

impl FrameBuffer {
    /// FrameBuffer 0, use this to bind on the default buffer
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            id: 0,
            texture: 0,
            unit: 0,
            depth: None,
            size: Vector2 {
                x: width as f32,
                y: height as f32,
            },
        }
    }

    pub fn new(width: usize, height: usize, unit: GLuint, depth_buffer: bool) -> Self {
        let mut id: GLuint = 0;
        let mut texture: GLuint = 0;
        let mut depth: GLuint = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);
            gl::DrawBuffer(gl::COLOR_ATTACHMENT0);

            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture,
                0,
            );

            if depth_buffer {
                gl::GenRenderbuffers(1, &mut depth);
                gl::BindRenderbuffer(gl::RENDERBUFFER, depth);
                gl::RenderbufferStorage(
                    gl::RENDERBUFFER,
                    gl::DEPTH_COMPONENT,
                    width as GLsizei,
                    height as GLsizei,
                );
                gl::FramebufferRenderbuffer(
                    gl::FRAMEBUFFER,
                    gl::DEPTH_ATTACHMENT,
                    gl::RENDERBUFFER,
                    depth,
                );
            }

            // unbind the current buffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Self {
            id,
            texture,
            unit,
            depth: if depth_buffer { Some(depth) } else { None },
            size: Vector2 {
                x: width as f32,
                y: height as f32,
            },
        }
    }

    pub fn clear(&self, color: bool, depth: bool, stencil: bool) {
        let mut bits = 0;

        if color {
            bits |= gl::COLOR_BUFFER_BIT;
        }

        if depth {
            bits |= gl::DEPTH_BUFFER_BIT;
        }

        if stencil {
            bits |= gl::STENCIL_BUFFER_BIT;
        }

        unsafe {
            gl::Clear(bits);
        }
    }

    pub fn size(&self) -> Vector2 {
        self.size
    }

    pub fn unit(&self) -> GLuint {
        self.unit
    }

    pub fn bind_texture(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }
}

impl Identifiable for FrameBuffer {
    type Id = gl::types::GLuint;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Bindable for FrameBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}
