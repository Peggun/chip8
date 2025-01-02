extern crate sdl2;

use sdl2::sys::{SDL_CreateRenderer, SDL_CreateTexture, SDL_CreateWindow, SDL_DestroyRenderer, SDL_DestroyTexture, SDL_DestroyWindow, SDL_Event, SDL_Init, SDL_PollEvent, SDL_Quit, SDL_RenderClear, SDL_RenderCopy, SDL_RenderPresent, SDL_Renderer, SDL_Texture, SDL_UpdateTexture, SDL_Window, SDL_WindowFlags, SDL_INIT_VIDEO};

pub const SDL_RENDERER_ACCELERATED: u32 = 0x00000002;
pub const SDL_PIXELFORMAT_RGBA8888: u32 = 0x16762004;
pub const SDL_TEXTUREACCESS_STREAMING: u32 = 0x00000001;

pub struct Platform {
    pub window: *mut SDL_Window,
    pub renderer: *mut SDL_Renderer,
    pub texture: *mut SDL_Texture,
}

impl Platform {
    pub fn new(
        title: &str,
        window_width: i32,
        window_height: i32,
        texture_width: i32,
        texture_height: i32,
    ) -> Self {
        unsafe {
            SDL_Init(SDL_INIT_VIDEO);

            let window = SDL_CreateWindow(
                std::ffi::CString::new(title).unwrap().as_ptr(),
                0,
                0,
                window_width,
                window_height,
                SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
            );

            let renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED as u32);

            let texture = SDL_CreateTexture(
                renderer,
                SDL_PIXELFORMAT_RGBA8888 as u32,
                SDL_TEXTUREACCESS_STREAMING as i32,
                texture_width,
                texture_height,
            );

            Platform {
                window,
                renderer,
                texture,
            }
        }
    }

    pub fn update(&mut self, buffer: *const std::ffi::c_void, pitch: i32) {
        unsafe {
            SDL_UpdateTexture(self.texture, std::ptr::null(), buffer, pitch);
            SDL_RenderClear(self.renderer);
            SDL_RenderCopy(
                self.renderer,
                self.texture,
                std::ptr::null(),
                std::ptr::null(),
            );
            SDL_RenderPresent(self.renderer);
        }
    }
}

impl Drop for Platform {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyTexture(self.texture);
            SDL_DestroyRenderer(self.renderer);
            SDL_DestroyWindow(self.window);
            SDL_Quit();
        }
    }
}