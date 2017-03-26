use std::collections::HashMap;

use na::Vector3;
use ncollide::shape::Ball;
use nphysics3d::world::World;
use nphysics3d::object::RigidBody;
use time;

use game::board::{Board, PlayerId};
use game::ship::Ship;

pub struct Round {
    last_tick: f64,
    pub board: Board,
    world: World<f32>,
    bodies: HashMap<PlayerId, RigidBody<f32>>,
}

const TIMESTEP_S: f64 = 0.01; // physics runs at 100 steps per second
const TICKS_TO_MS: u32 = 10;

impl Round {
    pub fn new() -> Round {
        let mut world = World::new();
        world.set_gravity(Vector3::new(0.0, 0.0, 0.0));

        Round {
            last_tick: time::precise_time_s(),
            board: Board::new(),
            world: world,
            bodies: HashMap::new(),
        }
    }

    pub fn add_ship(&mut self, player: PlayerId, ship: Ship) {
        // TODO: magic numbers
        // TODO: figure out the real shape
        let rad = 0.5;
        let mut rb: RigidBody<f32> = RigidBody::new_dynamic(Ball::new(rad), 1.0, 0.3, 0.6);
        rb.append_translation(&ship.translation());
        self.bodies.insert(player, rb);
        self.board.add_ship(player, ship);
    }

    pub fn fire_engine(&mut self, player: PlayerId, vector: Vector3<f32>) {
        // TODO: this function should take into account which way the ship is pointing
        self.bodies.get_mut(&player)
            .map(|rb| { rb.apply_central_impulse(vector) })
            .or_else(|| {
                println!("No rigid body registered for {}", player);
                None
            });
    }

    /// Advance the physics world by as much time as has elapsed since the last tick
    /// Always steps the world ahead at 100fps, may make multiple steps per call
    pub fn tick(&mut self) -> u32 {
        let ticks = (self.dt_s() / TIMESTEP_S) as u32;
        self.tick_ahead(ticks)
    }

    fn tick_ahead(&mut self, ticks: u32) -> u32 {
        for _ in 0..ticks  {
            self.world.step(TIMESTEP_S as f32);
        }
        self.last_tick += ticks as f64 * TIMESTEP_S;
        self.board.advance(ticks * TICKS_TO_MS);
        ticks
    }

    fn dt_s(&self) -> f64 {
        let now = time::precise_time_s();
        now - self.last_tick
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;
    use nphysics3d::math::Point;
    use super::*;

    #[test]
    fn test_add_ship() {
        let mut round = Round::new();
        let ship = Ship::at_origin();
        round.add_ship(2, ship);
        assert_eq!(1, round.bodies.len());
        assert_eq!(1, round.board.ships.len());
    }

    #[test]
    fn test_dt() {
        let round = Round::new();
        thread::sleep(Duration::from_millis(50));
        let dt = round.dt_s();
        println!("got dt {}", dt);
        assert!(dt >= 0.05 && dt < 0.06);
    }

    #[test]
    fn test_tick() {
        let mut round = Round::new();
        let last_ticked = round.last_tick;
        thread::sleep(Duration::from_millis(42)); // 4 full frames plus some slop
        assert_eq!(last_ticked, round.last_tick); // last_tick not advanced by time alone
        let ticks = round.tick();
        assert_eq!(ticks, 4);
        assert!(round.last_tick >= ticks as f64 * TIMESTEP_S + last_ticked);
    }

    #[test]
    fn physics_even() {
        let mut round = Round::new();
        round.add_ship(1, Ship::at_origin());
        {
            let ship = round.bodies.get(&1).expect("couldn't find ship");
            assert!(ship.can_move());
            assert_eq!(ship.position_center(), Point::new(0.0, 0.0, 0.0));
            assert_eq!(ship.lin_vel(), Vector3::new(0.0, 0.0, 0.0));
            // TODO: test position at origin
        }

        round.fire_engine(1, Vector3::new(1.0, 0.0, 0.0));
        round.tick_ahead(100); // run for 1 second

        {
            let ship = round.bodies.get(&1).expect("couldn't find ship");
            let vel = ship.lin_vel();
            assert!(vel.x > 0.0, "x vel greater than 0: {}", vel.x);
            assert_eq!(vel.y, 0.0, "y vel");
            assert_eq!(vel.z, 0.0, "z vel");
            // TODO: test position moved
        }
    }
}
