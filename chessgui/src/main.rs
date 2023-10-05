use ggez::conf::{WindowMode, WindowSetup};
use ggez::glam::{Vec2};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Canvas, Rect, DrawParam};
use ggez::event::{self, EventHandler};
use sagakar_chess_lib::{self, Game};
use std::{env, path};

enum GameState {
    InGame,
    Ended,
}
struct MainState {
    game: Game,
    state: GameState,
    selected: Vec2,
    is_select: bool,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        _ctx.gfx.add_font(
            "arial",
            graphics::FontData::from_path(_ctx, "/arial-unicode-ms.ttf")?,
        );

        Ok(MainState {
            game: Game::new(),
            state: GameState::InGame,
            selected: Vec2::new(0., 0.),
            is_select: false,
        })

    }

    fn update_board(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(_ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));

        let (wwin, hwin) = _ctx.gfx.drawable_size();
        let game_board = self.game.get_board();

        for i in 0..8 {
            for j in 0..8 {
                let position = Vec2::new(
                    wwin/8. * j as f32,
                    hwin/8. * i as f32,
                );
                
                let mut piece_type: &str = "";
                let mut piece_colour: &str = "";
                if game_board[i][j].is_some() {
                    piece_type = match game_board[i][j].unwrap() {
                        sagakar_chess_lib::Piece::King => "♚",
                        sagakar_chess_lib::Piece::Bishop => "♝",
                        sagakar_chess_lib::Piece::Knight => "♞",
                        sagakar_chess_lib::Piece::Pawn => "♟",
                        sagakar_chess_lib::Piece::Queen => "♛",
                        sagakar_chess_lib::Piece::Rook => "♜",
                        _ => "",
                    };

                    piece_colour = match self.game.get_color_at(j, i).unwrap() {
                        sagakar_chess_lib::Color::Black => "black",
                        sagakar_chess_lib::Color::White => "white",
                    };
                };

                canvas.draw(
                    &graphics::Mesh::new_rectangle(
                        _ctx,
                        graphics::DrawMode::fill(),
                        Rect::new(0., 0., wwin/8., hwin/8.),
                        if (i + j) % 2 == 0 {
                            graphics::Color::from_rgb(255, 161, 161)
                        } else {
                            graphics::Color::from_rgb(180, 112, 80)
                        }
                    )?,
                    position
                );

                

                canvas.draw(
                    graphics::Text::new(piece_type)
                        .set_scale(wwin/8.)
                        .set_font("arial"), 
                    DrawParam::default().dest(position).color(if piece_colour == "white" {
                        graphics::Color::from_rgb(255, 255, 255)
                    } else {
                        graphics::Color::from_rgb(0, 0, 0)
                }));
            }
        }

        /*let piece_colour = match self.game.get_color_at(self.selected.x as usize, self.selected.y as usize).unwrap() {
            sagakar_chess_lib::Color::Black => "black",
            sagakar_chess_lib::Color::White => "white",
        };
        let active_colour = match self.game.player {
            sagakar_chess_lib::Color::Black => todo!(),
            sagakar_chess_lib::Color::White => todo!(),
        };*/

        if self.is_select {
            if self.game.get_possible_moves(&coordinates_to_string(self.selected.x as usize, self.selected.y as usize)).is_some() {
                if true {
                    for g in self.game.get_possible_moves(&coordinates_to_string(self.selected.x as usize, self.selected.y as usize)).unwrap() {
                        canvas.draw(
                        &graphics::Mesh::new_rectangle(
                                _ctx,
                                graphics::DrawMode::fill(),
                                Rect::new(0., 0., wwin/8., hwin/8.),
                                if game_board[string_to_coordinates(&g).1][string_to_coordinates(&g).0] != None {
                                    graphics::Color::from_rgba(100, 69, 200, 200)
                                } else {
                                    graphics::Color::from_rgba(182, 212, 119, 200)
                                }
                            )?,
                            Vec2::new((string_to_coordinates(&g).0 as f32) * wwin/8., (string_to_coordinates(&g).1 as f32) * hwin/8.)
                        );
                    }
                };
            }
        };

        if self.game.get_game_state() == sagakar_chess_lib::GameState::Checkmate {
            canvas.draw(graphics::Text::new("Checkmate :O").set_scale(100.).set_font("arial"), DrawParam::default().dest(Vec2::new(0., 0.)).color(graphics::Color::from_rgb(255, 255, 255)));
            self.state = GameState::Ended;
        };

        canvas.finish(_ctx);
        return Ok(());
    }

    fn update_mouse_thing(&mut self, _ctx: &mut Context, x: f32, y: f32) -> GameResult {

        let (wwin, hwin) = _ctx.gfx.drawable_size();
        let x_pos = (x / (wwin/8.)).floor() as usize;
        let y_pos = (y / (hwin/8.)).floor() as usize;

        if !self.is_select {
            self.is_select = true;
            self.selected = Vec2::new(x_pos as f32, y_pos as f32);
            return Ok(());
        } else {
            let mut m_list: Vec<String> = vec![];
            if self.game.get_possible_moves(&coordinates_to_string(self.selected.x as usize, self.selected.y as usize)).is_some() {
                m_list = self.game.get_possible_moves(&coordinates_to_string(self.selected.x as usize, self.selected.y as usize)).unwrap();
                if m_list.contains(&coordinates_to_string(x_pos, y_pos).to_string()) {
                    self.game.make_move(&coordinates_to_string(self.selected.x as usize, self.selected.y as usize), &coordinates_to_string(x_pos, y_pos));
                }
            };
            self.is_select = false;
            return Ok(());
        }
    }

}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        
        return Ok(());
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        return match self.state {
            GameState::InGame => self.update_board(_ctx),
            _ => Ok(()),
        }
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> GameResult {
        match self.state {
            GameState::InGame => self.update_mouse_thing(_ctx, _x, _y),
            _ => Ok(()),
        }
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("helloworld", "ggez").add_resource_path(resource_dir).window_mode(WindowMode::default().dimensions(800., 800.)).window_setup(WindowSetup::default().title("slay queen"));
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

fn string_to_coordinates(position: &str) -> (usize, usize) {
    let mut x = position.chars().nth(0).unwrap();
    x.make_ascii_uppercase();
    let x = usize::from((x as u8) - 65); // Turn x into an integer by casting char to u8 and removing the ASCII offset

    let y = position.chars().nth(1).unwrap();
    let y = usize::from(8 - (y as u8 - 48)); // Same as x, but subtract from 8 to uninvert y coordinate

    return (x, y)
}

fn coordinates_to_string(x: usize, y: usize) -> String {
    let x = u8::try_from(x).unwrap();
    let x = (x + 65) as char;

    let y = u8::try_from(y).unwrap();
    let y = (8 - y + 48) as char;
    return String::from(x) + &String::from(y)
}