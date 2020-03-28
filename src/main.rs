use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::conf;
use std::path;

mod network;

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("Wheel Client", "Esbysie")
            .window_setup(conf::WindowSetup::default().title("Wheel Client"))
            .window_mode(conf::WindowMode::default().dimensions(1000.0, 800.0))   
            .add_resource_path(path::PathBuf::from("resources"))
            .build()
            .unwrap();
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        std::thread::spawn(move || {
            network::net_main();
        });
       
        MyGame { }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // Draw code here...

        graphics::present(ctx)
    }
}
