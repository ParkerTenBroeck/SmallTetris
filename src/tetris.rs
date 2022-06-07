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
        pub piece: Option<FallingPiece>,
        pub board: Board,
        pub score: usize,
    }

    impl TetrisGame{
        pub fn get_curr_piece(&self) -> Option<(u8, [Coord; 4])>{
            match &self.piece{
                Some(val) => {
                    let t = val.piece_type.get_coords(val.rotation);
                    let mut coords = [[0,0i16].into(); 4];
                    for i in 0..4{
                        coords[i] = val.coords + t[i].into();
                    }
                
                    Option::Some((val.piece_type.as_num(), coords))
                },
                None => Option::None,
            }
        }

        pub fn get_dropped_piece(&self) -> Option<(u8, [Coord; 4])>{
            match &self.piece{
                Some(val) => {
                    let t = val.piece_type.get_coords(val.rotation);
                    let mut coords = [[0,0i16].into(); 4];
                    for i in 0..4{
                        coords[i] = val.coords + t[i].into();
                    }
                
                    Option::Some((val.piece_type.as_num(), coords))
                },
                None => Option::None,
            }
        }
    }
    pub struct FallingPiece{
        piece_type: Tetrominoes,
        rotation: u8,
        coords: Coord,
    }

    pub struct Board{
        data: [u32; 40]
    }

    impl Board{
        fn new() -> Self{
            Self { data: [12; 40] }
        }
        #[inline(always)]
        fn is_empty(&self, coord: Coord) -> bool{
            self.data_at_coord(coord) == 0
        }
        #[inline(always)]
        fn is_full(&self, coord: Coord) -> bool{
            !self.is_empty(coord)
        }
        #[inline(always)]
        pub fn data_at_coord(&self, coord: Coord) -> u8{
            ((self.data[coord.y as usize] >> (coord.x * 3)) & 7) as u8 
        }
        #[inline(always)]
        fn set_data_at_coord(&mut self, data: u8, coord: Coord){
            let y = &mut self.data[coord.y as usize];
            *y = (*y & (!(7 << (coord.x as u32 * 3)))) | ((data as u32 & 7) << (coord.x * 3))
        }
    }

    impl TetrisGame{
        pub fn init() -> Self{
            Self { 
                piece: Option::None,
                board: Board::new(),
                score: 0,  
            }
        }
    }

    #[derive(Default, Copy, Clone)]
    pub struct Coord{
        pub x: i16,
        pub y: i16
    }

    impl Coord{
        pub fn scale(&self, scaler: i16) -> Self{
            Coord{x: self.x * scaler, y: self.y * scaler}
        }
    }

    impl From<[i16; 2]> for Coord{
        fn from(coord: [i16; 2]) -> Self {
            Coord { x: coord[0], y: coord[1] }
        }
    }

    impl From<[u8; 2]> for Coord{
        fn from(coord: [u8; 2]) -> Self {
            Coord { x: coord[0] as i16, y: coord[1] as i16 }
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
    impl From<Coord> for [i16; 2]{
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
            rotation_matrix[rotation as usize][self.as_num() as usize]
        }

        pub fn as_num(&self) -> u8{
            match self{
                Tetrominoes::I => 0,
                Tetrominoes::J => 1,
                Tetrominoes::L => 2,
                Tetrominoes::O => 3,
                Tetrominoes::S => 4,
                Tetrominoes::T => 5,
                Tetrominoes::Z => 6,
            }
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
                self.draw_cube([x,0i16].into(), &TETROMINOE_PALLETE[7]);
            }
            for x in 0..12{
                self.draw_cube([x,21i16].into(), &TETROMINOE_PALLETE[7]);
            }
            for y in 1..21{
                self.draw_cube([0i16,y].into(), &TETROMINOE_PALLETE[7]);
            }
            for y in 1..21{
                self.draw_cube([11i16,y].into(), &TETROMINOE_PALLETE[7]);
            }

            for x in 0u8..10{
                for y in 0u8..20{
                    let data = self.game.board.data_at_coord([x,y + 20].into());
                    if data == 0{
                        self.fill_cube([x+1,y+1].into(), Color::from_rgb(0, 0, 0));
                    }else{
                        self.ghost_cube([x+1, y+1].into(), TETROMINOE_PALLETE[data as usize][0]);
                    }
                }
            }

            match self.game.get_curr_piece(){
                Some(piece) => {
                    for coord in piece.1{
                        self.draw_cube(coord + [1,1i16].into(), &TETROMINOE_PALLETE[piece.0 as usize])
                    }
                },
                None => {},
            }

            self.interface.update_screen();
            self.frame_counter += 1;
        }

        fn draw_cube(&mut self, coords: Coord, cube_pallete: &[Color; 5]){
            let start_x = coords.x as usize * 8;
            let start_y = coords.y as usize * 8;

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
                }
            }
        }

        fn ghost_cube(&mut self, location: Coord, color: Color){

            if !color.is_opaque(){
                return;
            }
            let location = location.scale(8);
            for x in 0..8{
                self.interface.set_pixel(location.x as usize + x, location.y as usize, color);
                self.interface.set_pixel(location.x as usize + x, location.y as usize + 7, color);
            }
            for y in 1..7{
                self.interface.set_pixel(location.x as usize, location.y as usize + y, color);
                self.interface.set_pixel(location.x as usize + 7, location.y as usize + y, color);
            }
        }

        fn draw_tile(&mut self, location: Coord, tile: &[u8; 256], pallet: &[Color]){
            for x in 0..8{
                for y in 0..8{
                    let color = pallet[tile[x + y*8] as usize];
                    if color.is_opaque(){
                        self.interface.set_pixel(x + (location.x*8) as usize, y + (location.y*8) as usize, color);
                    }
                }
            }
        }

        fn fill_cube(&mut self, coords: Coord, color: Color){
            let start_x = coords.x as usize * 8;
            let start_y = coords.y as usize * 8;

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


