#![cfg_attr(target_arch = "mips", no_std)]
#![cfg_attr(target_arch = "mips", feature(lang_items, start))]
#![cfg_attr(target_arch = "mips", no_builtins)]


pub mod tetris;


#[cfg(not(target_arch = "mips"))]
pub fn main() {
    let native_options = eframe::NativeOptions{
        ..Default::default()
    };

    eframe::run_native("Tetris", native_options, Box::new(|cc|{
        let app = app::TetrisWindow::new(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(eframe::egui::Visuals::dark());
        Box::new(app)
    }));
}

#[cfg(target_arch = "mips")]
#[no_mangle]
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize{
    use tetris::Tetris;

    use crate::{platform::Interface, util::sleep_delta_mills};

    let interface = Interface{

    };

    let mut tetris = Tetris::init(interface);
    loop{
        if !tetris.run_frame(){
            break;
        }
        sleep_delta_mills(17);
    }
    0
}


#[cfg(target_arch = "mips")]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}



#[cfg(target_arch = "mips")]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


//-------------------------------------------------- util

#[cfg(not(target_arch = "mips"))]
mod util{
    pub type Color = eframe::egui::Color32;
}

#[cfg(target_arch = "mips")]
mod util{
    #[allow(dead_code)]
    #[derive(Copy, Clone)]
    pub struct Color([u8; 4]);

    impl Color{
        #[inline(always)]
        pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
            Self([r, g, b, 255])
        }
    
        #[inline(always)]
        pub const fn from_rgb_additive(r: u8, g: u8, b: u8) -> Self {
            Self([r, g, b, 0])
        }

        pub fn is_opaque(&self) -> bool {
            self.0[3] == 255
        }
    }

    pub fn sleep_delta_mills(_mills: usize){

    }
}

//-------------------------------------------------- util

trait InterfaceTrait{
    fn update_screen(&mut self);
    fn set_pixel(&mut self, x: usize, y: usize, color: util::Color);
    fn clear_screen(&mut self, color: util::Color);
    fn key_down(&mut self, key: char) -> bool;
}

//-------------------------------------------------- Desktop


#[cfg(not(target_arch = "mips"))]
mod app{
    use eframe::epaint::{ColorImage, Color32, Rounding, TextureId};

    use crate::{tetris::{Tetris, renderer::*}, platform::Interface};
    
    pub struct TetrisWindow{
        game: Tetris,
        frame_id: TextureId,
    }
    impl TetrisWindow {
        pub fn new(ctx: &eframe::egui::Context) -> TetrisWindow{
            let frame = ColorImage::new([WIDTH, HEIGHT], Color32::BLACK);
            let frame_handler = ctx.load_filtered_texture("tetris_window", frame.clone(),eframe::epaint::textures::TextureFilter::Nearest);
            
            let interface = Interface{
                frame_handler: frame_handler.clone(),
                frame,
                context: ctx.clone(),
            };
    
            TetrisWindow { 
                game: Tetris::init(interface),
                frame_id: frame_handler.id(),
            }
        }
    }
    
    impl eframe::App for TetrisWindow{
        fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
            if !self.game.run_frame(){
                frame.quit()
            }
            let frame = eframe::egui::Frame { 
                inner_margin: eframe::egui::style::Margin::same(0.0), 
                outer_margin: eframe::egui::style::Margin::same(0.0), 
                rounding: Rounding::none(), 
                shadow: eframe::epaint::Shadow::default(), 
                fill: Default::default(), 
                stroke: Default::default() 
            };
            eframe::egui::CentralPanel::default().frame(frame).show(ctx, |ui|{
                let mut width = ui.available_width();
                let mut height = width / WIDTH as f32 * HEIGHT as f32; 
    
                if height > ui.available_height(){
                    height = ui.available_height(); 
                    width = height / HEIGHT as f32 * WIDTH as f32;
                }
    
    
                ui.image(self.frame_id, [width, height]);
            });
            ctx.request_repaint();
        
        }
    }
}

#[cfg(not(target_arch = "mips"))]
pub mod platform{
    use eframe::{epaint::{ColorImage, TextureHandle}, egui::Context};

    use crate::{InterfaceTrait, tetris::renderer::*};

    pub struct Interface{
        pub frame_handler: TextureHandle,
        pub frame: ColorImage,
        pub context: Context
    }

    impl InterfaceTrait for Interface{
        #[inline(always)]
        fn update_screen(&mut self) {
            self.frame_handler.set(self.frame.clone(), eframe::epaint::textures::TextureFilter::Nearest)
        }
        #[inline(always)]
        fn set_pixel(&mut self, x: usize, y: usize, color: crate::util::Color) {
            self.frame.pixels[x + y * self.frame.size[0]] = color;
        }
        #[inline(always)]
        fn clear_screen(&mut self, color: crate::util::Color) {
            self.frame = ColorImage::new([WIDTH, HEIGHT], color);
        }
        #[inline(always)]
        fn key_down(&mut self, key: char) -> bool {
            use eframe::egui::Key::*;
            self.context.input().key_down(match key{
                'A'|'a' => A,
                'B'|'b' => B,
                'C'|'c' => C,
                'D'|'d' => D,
                'E'|'e' => E,
                'F'|'f' => F,
                'G'|'g' => G,
                'H'|'h' => H,
                'I'|'i' => I,
                'J'|'j' => J,
                'K'|'k' => K,
                'L'|'l' => L,
                'M'|'m' => M,
                'N'|'n' => N,
                'O'|'o' => O,
                'P'|'p' => P,
                'Q'|'q' => Q,
                'R'|'r' => R,
                'S'|'s' => S,
                'T'|'t' => T,
                'U'|'u' => U,
                'V'|'v' => V,
                'W'|'w' => W,
                'X'|'x' => X,
                'Y'|'y' => Y,
                'Z'|'z' => Z,
                '0' => Num0,
                '1' => Num1,
                '2' => Num2,
                '3' => Num3,
                '4' => Num4,
                '5' => Num5,
                '6' => Num6,
                '7' => Num7,
                '8' => Num8,
                '9' => Num9,
                ' ' => Space,
                '\n' => Enter,
                '\x26' => ArrowUp,
                '\x25' => ArrowLeft,
                '\x27' => ArrowDown,
                '\x28' => ArrowRight,
                _ => panic!()
            })
        }
    }
}


//-------------------------------------------------- Desktop

#[cfg(target_arch = "mips")]
pub mod platform{
    use crate::InterfaceTrait;

    pub struct Interface{

    }

    pub fn black_box<T>(dummy: T) -> T{
        unsafe {
            let ret = core::ptr::read_volatile(&dummy);
            core::mem::forget(dummy);
            ret
        }
    }

    #[allow(unused)]
    impl InterfaceTrait for Interface{
        #[inline(always)]
        fn update_screen(&mut self) {
            black_box(self);
        }
        #[inline(always)]
        fn set_pixel(&mut self, x: usize, y: usize, color: crate::util::Color) {
            black_box(self);
            black_box(x);
            black_box(y);
            black_box(color);
        }
        #[inline(always)]
        fn clear_screen(&mut self, color: crate::util::Color) {
            black_box(self);
            black_box(color);
        }
        #[inline(always)]
        fn key_down(&mut self, key: char) -> bool {
            black_box(self);
            black_box(key);
            black_box(false)
        }
    }
}