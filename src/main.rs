use std::fmt;
use std::fmt::{Display, Formatter, Error };

enum VerticalDir{
    Up,
    Down
}
enum HorizontalDir{
    Left,
    Right
}
struct Ball{
    x: i32,
    y: i32,
    x_dir: HorizontalDir,
    y_dir: VerticalDir
}
struct Frame{
    width: i32,
    height: i32
}
struct Game{
    frame: Frame,
    ball: Ball
}
impl Game{
    fn new()->Game{
        Game{
            frame:Frame{
                width: 63,
                height: 61
            },
            
            ball:Ball{
                x: 44,
                y: 21,
                x_dir: HorizontalDir::Right,
                y_dir: VerticalDir::Down
            }
        }
    }
    fn step(&mut self){
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}
impl Ball{
    fn bounce(&mut self, frame: &Frame){
        if self.x<=0{
            self.x_dir = HorizontalDir::Right;
        }
        else if frame.width <= self.x{
            self.x_dir = HorizontalDir::Left;
        }
        else if self.y <= 0{
            self.y_dir = VerticalDir::Down;
        }
        else if frame.height <= self.y{
            self.y_dir = VerticalDir::Up;
        }
        else{}
    }
    fn mv(&mut self){
        match self.x_dir{
            HorizontalDir::Left => self.x -= 1,
            HorizontalDir::Right => self.x += 1
        }
        match self.y_dir{
            VerticalDir::Up => self.y -= 1,
            VerticalDir::Down => self.y += 1
        }
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "x")?;
        for _ in 0..64 {
            write!(fmt, "-")?;
        }
        write!(fmt, "\n")?;
        for y in 0..32 {
            for x in 0..64 {
                if self.ball.x == x as i32 && self.ball.y == y as i32 {
                    write!(fmt, "0")?;
                } else if x == 0 {
                    write!(fmt, "!")?;
                } else if x != 0 && y != 31 {
                    write!(fmt, " ")?;
                } else {
                    write!(fmt, "-")?;
                }
            }
            write!(fmt, "\n")?;
        }
        write!(fmt, "\n")
    }
}

fn main() {
    let mut new_game = Game::new();
    let sleep_time  = std::time::Duration::from_millis(33);
    loop{
        new_game.step();
        println!("{}",new_game);
        std::thread::sleep(sleep_time);
        println!("{} {} ",new_game.ball.x,new_game.ball.y)
    }
}
