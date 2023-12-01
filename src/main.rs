use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 100;
const SCREEN_HEIGHT: i32 = 70;
const FRAME_DURATION: f32 = 60.0;

struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    obstacles: Vec<Obstacle>,
    score: i32,
    frame_count: i32,
}

impl State {
    fn new() -> Self {
        let mut obstecles = Vec::new();
        obstecles.push(Obstacle::new(SCREEN_WIDTH, 0));

        Self {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacles: obstecles,
            score: 0,
            frame_count: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.frame_count += 1;

            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        for o in &self.obstacles {
            o.clone().render(ctx, self.player.x);
        }

        for (pos, o) in self.obstacles.clone().iter().enumerate() {
            if o.hit_obstacle(&self.player) {
                self.mode = GameMode::End
            }

            if self.player.x > o.x {
                self.score += 1;
                self.obstacles.remove(pos);
            }
        }

        if self.frame_count % 60 == 0 {
            self.obstacles
                .push(Obstacle::new(self.player.x, self.score));
        }

        if self.player.y + self.player.height / 2 > SCREEN_HEIGHT {
            self.mode = GameMode::End
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(SCREEN_HEIGHT / 2, BLACK, RED, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Restart Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(SCREEN_HEIGHT / 2, GREEN, BLACK, "Welcome to Flappy Dragon");
        ctx.print_centered(SCREEN_HEIGHT / 2 + 2, "(P) Play Game");
        ctx.print_centered(SCREEN_HEIGHT / 2 + 4, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self, _ctx: &mut BTerm) {
        let mut obstecles = Vec::new();

        for _i in 0..5 {
            obstecles.push(Obstacle::new(self.player.x, 0));
        }

        self.player = Player::new(5, 25);
        self.obstacles = obstecles;
        self.score = 0;
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
    width: i32,
    height: i32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
            width: 1,
            height: 1,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.draw_box(5, self.y, self.width, self.height, YELLOW, GREEN);
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

#[derive(Debug, Clone)]
struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(player_x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x: player_x + random.range(SCREEN_WIDTH, SCREEN_WIDTH * 2),
            gap_y: random.range(10, 40),
            size: i32::max(10, 50 - score),
        }
    }

    fn render(self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        // Draw the top half of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.draw_bar_horizontal(screen_x, y, 2, 1, 1, RED, RED)
        }
        // Draw the bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.draw_bar_horizontal(screen_x, y, 2, 1, 1, RED, RED)
            // ctx.draw_box(screen_x, y, 2, 2, RED, BLACK)
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let player_half_size = player.height / 2;
        let does_x_match = (player.x + player_half_size) == self.x;
        let player_above_gap = (player.y - player_half_size) < self.gap_y - half_size;
        let player_below_gap = (player.y + player_half_size) > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}
enum GameMode {
    Menu,
    Playing,
    End,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Flappy Dragon")
        .with_tile_dimensions(12, 12)
        .build()?;

    main_loop(context, State::new())
}
