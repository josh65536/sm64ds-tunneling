use crate::fix::Fix;
use crate::fx;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Wait,
    Jump,
    FlutterJump,
}

#[derive(Clone, Debug)]
pub struct Yoshi {
    position_y: Fix,
    velocity_y: Fix,
    vert_accel: Fix,
    terminal_velocity: Fix,
    double_jump: bool,
    can_flutter_jump: bool,
    state: State,
}

impl Yoshi {
    const VERT_ACCEL: f64 = -4.0;
    const TERMINAL_VELOCITY: f64 = -75.0;

    const JUMP_FACTOR: f64 = 0.89990234375; // 3686/4096
	const JUMP_UP_VERT_ACCEL: f64 = -8.0;
	const JUMP_UP_VERT_ACCEL_YOSHI_HOLDING_B: f64 = -3.0;

    const FLUTTER_JUMP_MAX_START_VERT_SPEED: f64 = -8.0;
    const FLUTTER_JUMP_UP_VERT_ACCELERATION: f64 = 1.0;
    const FLUTTER_JUMP_DOWN_VERT_ACCELERATION: f64 = 0.75;
    const FLUTTER_JUMP_MAX_VERT_SPEED: f64 = 17.0;
	
    pub fn new(double_jump: bool) -> Self {
        Self {
            position_y: fx!(0.0),
            velocity_y: fx!(0.0),
            vert_accel: fx!(Self::VERT_ACCEL),
            terminal_velocity: fx!(Self::TERMINAL_VELOCITY),
            double_jump,
            can_flutter_jump: true,
            state: State::Wait,
        }
    }

    pub fn position_y(&self) -> Fix {
        self.position_y
    }

    pub fn update(&mut self, holding_b: bool) {
        match self.state {
            State::Wait => {
                // Assume b is pressed here
                self.velocity_y = fx!(if self.double_jump {52.0} else {42.0}) * fx!(Self::JUMP_FACTOR);
                self.state = State::Jump;
            }

            State::Jump => {
                self.vert_accel = if self.velocity_y < fx!(0.0) {
                    90 * fx!(Self::VERT_ACCEL) / 100
                } else if holding_b {
                    90 * fx!(Self::JUMP_UP_VERT_ACCEL_YOSHI_HOLDING_B) / 100
                } else {
                    fx!(Self::JUMP_UP_VERT_ACCEL)
                };

                if holding_b && self.can_flutter_jump && self.velocity_y < fx!(Self::FLUTTER_JUMP_MAX_START_VERT_SPEED) {
                    self.state = State::FlutterJump;
                    self.vert_accel = fx!(0.0);
                }
            }

            State::FlutterJump => {
                if holding_b {
                    self.velocity_y += if self.velocity_y >= fx!(0.0) {
                        fx!(Self::FLUTTER_JUMP_UP_VERT_ACCELERATION)
                    } else {
                        fx!(Self::FLUTTER_JUMP_DOWN_VERT_ACCELERATION)
                    }
                }

                if !holding_b || self.velocity_y >= fx!(Self::FLUTTER_JUMP_MAX_VERT_SPEED) {
                    self.state = State::Jump;
                    self.can_flutter_jump = false;
                }
            }
        }

        self.velocity_y = (self.velocity_y + self.vert_accel).max(self.terminal_velocity);
        self.position_y += self.velocity_y;
    }
}