use self::{game::TetrisGame, renderer::TetrisRenderer, input::TetrisInput, sound::TetrisSound};

#[allow(unused)]
pub struct Tetris{
    renderer: TetrisRenderer,
    game: TetrisGame,
    input: TetrisInput,
    sound: TetrisSound,
    frame_counter: usize,
    interface: crate::platform::Interface
}

impl Tetris{
    
    pub fn init(interface: crate::platform::Interface) -> Self{
        Tetris {
            interface,
            renderer: TetrisRenderer::init(),
            game: TetrisGame::init(),
            input: TetrisInput::init(),
            sound: TetrisSound::init(),
            frame_counter: 0,
        }
    }
    pub fn run_frame(&mut self) -> bool{
        self.update_input();
        self.update_sound();
        self.update_game();
        self.render_frame();
        self.frame_counter += 1;
        true
    }
}

mod game{
    use super::Tetris;


    pub struct TetrisGame{

    }
    impl TetrisGame{
        pub fn init() -> Self{
            Self {  }
        }
    }

    #[derive(Default, Copy, Clone)]
    pub struct Coord{
        pub x: u8,
        pub y: u8
    }

    impl From<[u8; 2]> for Coord{
        fn from(coord: [u8; 2]) -> Self {
            Coord { x: coord[0], y: coord[1] }
        }
    }
    impl core::ops::Add for Coord{
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            [self.x + rhs.x, self.y + rhs.y].into()
        }
    }
    impl core::ops::Sub for Coord{
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            [self.x - rhs.x, self.y - rhs.y].into()
        }
    }
    impl From<Coord> for [u8; 2]{
        fn from(coord: Coord) -> Self {
            [coord.x, coord.y]
        }
    }
    
    #[derive(Copy, Clone)]
    pub enum Tetrominoes{
        I,
        J,
        L,
        O,
        S,
        T,
        Z,
    }
    
    impl Tetrominoes{
        pub fn get_coords(&self, rotation: u8) -> [[u8; 2]; 4]{
            let rotation_matrix:[[[[u8; 2]; 4]; 8]; 4]  = [
                [
                    [[0,1],[1,1],[2,1],[3,1]],
                    [[0,0],[0,1],[1,1],[2,1]],
                    [[0,1],[1,1],[2,1],[2,0]],
                    [[0,0],[1,0],[0,1],[1,1]],
                    [[0,1],[1,1],[1,0],[2,0]],
                    [[0,1],[1,1],[2,1],[1,0]],
                    [[0,0],[1,0],[1,1],[2,1]],
                    [[0,0],[1,1],[2,2],[3,3]],//filler
                ],
                [
                    [[2,3],[2,2],[2,1],[2,0]],
                    [[2,0],[1,0],[1,1],[1,2]],
                    [[1,0],[1,1],[1,2],[2,2]],
                    [[0,0],[1,0],[0,1],[1,1]],
                    [[1,0],[1,1],[2,1],[2,2]],
                    [[1,2],[1,1],[1,0],[2,1]],
                    [[2,0],[2,1],[1,1],[1,2]],
                    [[3,0],[2,1],[1,2],[0,3]],//filler
                ],
                [
                    [[3,2],[2,2],[1,2],[0,2]],
                    [[0,1],[1,1],[2,1],[2,2]],
                    [[0,2],[0,1],[1,1],[2,1]],
                    [[0,0],[1,0],[0,1],[1,1]],
                    [[2,1],[1,1],[1,2],[0,2]],
                    [[2,1],[1,1],[0,1],[1,2]],
                    [[0,1],[1,1],[1,2],[2,2]],
                    [[0,0],[1,1],[2,2],[3,3]],//filler
                ],
                [
                    [[1,0],[1,1],[1,2],[1,3]],
                    [[1,0],[1,1],[1,2],[0,2]],
                    [[0,0],[1,0],[1,1],[1,2]],
                    [[0,0],[1,0],[0,1],[1,1]],
                    [[0,0],[0,1],[1,1],[1,2]],
                    [[1,0],[1,1],[1,2],[0,1]],
                    [[1,0],[1,1],[0,1],[0,2]],
                    [[3,0],[2,1],[1,2],[0,3]],//filler
                ],
            ];
            rotation_matrix[rotation as usize][
                match self{
                    Tetrominoes::I => 0,
                    Tetrominoes::J => 1,
                    Tetrominoes::L => 2,
                    Tetrominoes::O => 3,
                    Tetrominoes::S => 4,
                    Tetrominoes::T => 5,
                    Tetrominoes::Z => 6,
                }
            ]
        }
    }

    impl Tetris{
        pub fn update_game(&self){

        }
    }
    
}

pub mod renderer{

    pub const WIDTH: usize = 12*8;
    pub const HEIGHT: usize = 22*8;

    use crate::{tetris::game::Coord, InterfaceTrait, util::Color};

    use super::{Tetris};

    pub struct TetrisRenderer{

    }

    impl TetrisRenderer{
        pub fn init() -> Self {
            Self {
            }
        }   
    }

    const TETROMINOE_PALLETE: [[Color; 5]; 8] = [
        [
            Color::from_rgb(18, 255, 255),
            Color::from_rgb(179, 255, 255),
            Color::from_rgb(0, 179, 179),
            Color::from_rgb(0, 76, 76),
            Color::from_rgb(76, 255, 255),
        ],
        [
            Color::from_rgb(0, 0, 179),
            Color::from_rgb(179, 179, 230),
            Color::from_rgb(0, 0, 119),
            Color::from_rgb(0, 0, 51),
            Color::from_rgb(76, 76, 196),
        ],
        [
            Color::from_rgb(255, 119, 0),
            Color::from_rgb(255, 214, 179),
            Color::from_rgb(179, 83, 0),
            Color::from_rgb(76, 35, 0),
            Color::from_rgb(255, 160, 76),
        ],
        [
            Color::from_rgb(255, 255, 0),
            Color::from_rgb(255, 255, 179),
            Color::from_rgb(179, 179, 0),
            Color::from_rgb(76, 76, 0),
            Color::from_rgb(255, 255, 76),
        ],
        [
            Color::from_rgb(23, 255, 0),
            Color::from_rgb(179, 255, 179),
            Color::from_rgb(13, 179, 0),
            Color::from_rgb(0, 76, 0),
            Color::from_rgb(79, 255, 79),
        ],
        [
            Color::from_rgb(204, 0, 204),
            Color::from_rgb(240, 179, 240),
            Color::from_rgb(142, 0, 142),
            Color::from_rgb(61, 0, 61),
            Color::from_rgb(219, 76, 219),
        ],
        [
            Color::from_rgb(255, 0, 0),
            Color::from_rgb(255, 179, 179),
            Color::from_rgb(179, 0, 0),
            Color::from_rgb(76, 0, 0),
            Color::from_rgb(255, 76, 76),
        ],
        [
            Color::from_rgb(119, 119, 119),
            Color::from_rgb(219, 219, 219),
            Color::from_rgb(82, 82, 82),
            Color::from_rgb(34, 34, 34),
            Color::from_rgb(160, 160, 160),
        ],
    ];

    impl Tetris{
        pub fn render_frame(&mut self){

            for x in 0..12{
                self.draw_cube([x,0], &TETROMINOE_PALLETE[7]);
            }
            for x in 0..12{
                self.draw_cube([x,21], &TETROMINOE_PALLETE[7]);
            }
            for y in 1..21{
                self.draw_cube([0,y], &TETROMINOE_PALLETE[7]);
            }
            for y in 1..21{
                self.draw_cube([11,y], &TETROMINOE_PALLETE[7]);
            }



            for x in 0..10{
                for y in 0..20{ //282589933 tiles??
                    // let mut h:usize = x+ y * 20;
                    // h = ((h >> 16) ^ h).wrapping_mul(0x45d9f3b);
                    // h = ((h >> 16) ^ h).wrapping_mul(0x45d9f3b);
                    // h = (x >> 16) ^ h;
                    // let h = h >> 1;

                    // self.draw_cube([x as u8+1,y as u8+1], palette[h % 7]);
                    self.fill_cube([x+1,y+1], Color::from_rgb(0, 0, 0)/*.linear_multiply(0.3)*/);
                }
            }

            unsafe{
                static mut ROT: usize = 0;
                static mut TET: usize = 0;
                static mut POS: Coord = Coord{x: 1, y: 1};

                if self.input.left_pressed(){
                    if POS.x > 0{
                        POS.x -= 1;
                    }
                }
                if self.input.right_pressed(){
                    POS.x += 1;
                }
                if self.input.down_pressed(){
                    POS.y += 1;
                }
                if self.input.up_pressed(){
                    if POS.y > 0{
                        POS.y -= 1;
                    }
                }
                if self.input.save_pressed(){
                    ROT += 1;
                }
                if self.input.drop_down_pressed(){
                    TET += 1;
                }

                ROT = ROT % 4;
                TET = TET % 7;
                
                use super::{game::Tetrominoes::*};
                let mut coords = [I,J,L,O,S,T,Z][TET].get_coords(ROT as u8);
                
                while{
                    let thing = ||{
                        for coord in coords{
                            let coord = POS + coord.into();
                            if coord.x >= 12{
                                POS.x -= 1;
                                return true;
                            }
                            if coord.y >= 22{
                                POS.y -= 1;
                                return true;
                            }
                        }
                        false
                    };
                    thing()
                }
                {
                    coords = [I,J,L,O,S,T,Z][TET].get_coords(ROT as u8);
                }

                for coord in coords{
                    self.draw_cube((POS + coord.into()).into(), &TETROMINOE_PALLETE[TET]);
                } 
            }

            self.interface.update_screen();
            self.frame_counter += 1;
        }

        fn draw_cube(&mut self, coords: [u8; 2], cube_pallete: &[Color; 5]){
            let start_x = coords[0] as usize * 8;
            let start_y = coords[1] as usize * 8;

            impl TrueColorMath for Color{
                fn lighten(self, _amount: f32) -> Color{
                    // let color = [self.r() as f32 / 255.0,self.g() as f32 / 255.0,self.b() as f32 / 255.0];
                    // let mut color = eframe::egui::color::Hsva::from_rgb(color);
                   
                    // let amount = amount + 1.0;

                    // color.s = (color.s / amount).clamp(0.0, 1.0);
                    // color.v = (color.v * amount).clamp(0.0, 1.0);
                    
                    // let color = color.to_rgb();
                    // Color::from_rgb((color[0] * 255.0) as u8, (color[1] * 255.0) as u8, (color[2] * 255.0) as u8)
                    self
                }
            }
            trait TrueColorMath{
                fn lighten(self, amount: f32) -> Self;
            }


            for x in 0..8{
                for y in 0..8{
                    let color = match (x,y){
                        (0..=6, 0) => {
                            //top light light
                            cube_pallete[1]
                        }
                        (1..=7, 7) => {
                            //bottom dark dark
                            cube_pallete[3]
                        }
                        (0, 1..=7) => {
                            //left light
                            cube_pallete[4]
                        }
                        (7, 0..=6) => {
                            //right dark
                            cube_pallete[2]
                        }
                        _ => {
                            cube_pallete[0]
                        }
                    };
                    if color.is_opaque(){
                        self.interface.set_pixel(x + start_x, y + start_y, color);
                    }
                    //self.renderer.frame.pixels[x + start_x + (y+start_y)*WIDTH] = color;
                }
            }
        }

        fn fill_cube(&mut self, coords: [u8; 2], color: Color){
            let start_x = coords[0] as usize * 8;
            let start_y = coords[1] as usize * 8;

            for x in 0..8{
                for y in 0..8{
                    if color.is_opaque(){
                        self.interface.set_pixel(x + start_x, y + start_y, color);
                    }
                }
            }
        }
    }
}


mod input{
    use crate::InterfaceTrait;

    use super::Tetris;

    pub struct TetrisInput{
        up: KeyState,
        left: KeyState,
        right: KeyState,
        down: KeyState,
        drop_down: KeyState,
        save: KeyState,
    }

    struct KeyState{
        key_down: bool,
        frames_down: usize,
        key_pressed: bool,
    }

    impl KeyState{
        fn new() -> Self{
            Self { 
                key_down: false, 
                frames_down: 0, 
                key_pressed: true 
            }
        }

        fn update(&mut self, down: bool){
            self.key_pressed = false;

            match (down, self.key_down){
                (true, true) => {
                    self.frames_down += 1;
                },
                (true, false) => {
                    self.key_down = true;
                    self.frames_down = 0;
                    self.key_pressed = true;
                },
                (false, true) => {
                    self.key_down = false;
                    self.frames_down = 0;
                },
                (false, false) => {

                },
            }
        }
    }

    impl TetrisInput{
        pub fn init() -> Self {
            Self {
                up: KeyState::new(),
                left: KeyState::new(),
                right: KeyState::new(),
                down: KeyState::new(),
                drop_down: KeyState::new(),
                save: KeyState::new(),
            }
        } 
    
        pub fn up_pressed(&self) -> bool{
            self.up.key_pressed
        }
        pub fn left_pressed(&self) -> bool{
            self.left.key_pressed
        }
        pub fn down_pressed(&self) -> bool{
            self.down.key_pressed
        }
        pub fn right_pressed(&self) -> bool{
            self.right.key_pressed
        }
        pub fn save_pressed(&self) -> bool{
            self.save.key_pressed
        }
        pub fn drop_down_pressed(&self) -> bool{
            self.drop_down.key_pressed
        }
    }

    impl Tetris{
        pub fn update_input(&mut self){

            self.input.up.update(self.interface.key_down('w') | self.interface.key_down('\x26'));
            self.input.left.update(self.interface.key_down('a') | self.interface.key_down('\x25'));
            self.input.down.update(self.interface.key_down('s') | self.interface.key_down('\x27'));
            self.input.right.update(self.interface.key_down('d') | self.interface.key_down('\x28'));
            self.input.save.update(self.interface.key_down('c'));
            self.input.drop_down.update(self.interface.key_down(' '));
            
            self.input.down.key_pressed = self.input.down.key_down;

            if self.input.left.frames_down > 20{
                self.input.left.key_pressed = self.input.left.frames_down % 8 == 0;
            }
            if self.input.right.frames_down > 20{
                self.input.right.key_pressed = self.input.right.frames_down % 8 == 0;
            }
        }
    }
}

mod sound{
    use super::Tetris;

    pub struct TetrisSound{
        
    }

    impl TetrisSound{
        pub fn init() -> Self {
            Self {
            }
        }
    
        
    }

    impl Tetris{
        pub fn update_sound(&self){
            
        }
    }
}


