#[cfg(test)]
mod tests {

    use fractal_draw::turtle_draw;
    use fractal_draw::types;

    const ANGLE_SHIFT: f64 = 45.0;

    const START_CURRENT: types::CurrentPosition = types::CurrentPosition {
        x: 0.0,
        y: 0.0,
        angle: 0.0,
    };

    #[test]
    fn m_step() {
        let input = &mut types::DrawPosition {
            current: START_CURRENT,
            memory: Vec::new(),
        };

        let expected_output = types::DrawPosition {
            current: types::CurrentPosition {
                x: 0.0,
                y: -100.0,
                angle: 0.0,
            },
            memory: Vec::new(),
        };

        let new_path = turtle_draw::step(input, 'm', ANGLE_SHIFT);

        // Path Changes
        assert_eq!(new_path, String::from(""));

        // Position changes
        assert!(input.current.angle - expected_output.current.angle < 1e-10);
        assert!(input.current.x - expected_output.current.x < 1e-10);
        assert!(input.current.y - expected_output.current.y < 1e-10);
    }

    #[test]
    fn l_step() {
        let input = &mut types::DrawPosition {
            current: START_CURRENT,
            memory: Vec::new(),
        };

        let expected_output = types::DrawPosition {
            current: types::CurrentPosition {
                x: 0.0,
                y: -100.0,
                angle: 0.0,
            },
            memory: Vec::new(),
        };

        let new_path = turtle_draw::step(input, 'l', ANGLE_SHIFT);

        // Path Changes
        assert_eq!(new_path, String::from(" M 0 0 L 0 -100"));

        // Position changes
        assert!(input.current.angle - expected_output.current.angle < 1e-10);
        assert!(input.current.x - expected_output.current.x < 1e-10);
        assert!(input.current.y - expected_output.current.y < 1e-10);
    }

    #[test]
    fn plus_step() {
        let input = &mut types::DrawPosition {
            current: START_CURRENT,
            memory: Vec::new(),
        };

        let expected_output = types::DrawPosition {
            current: types::CurrentPosition {
                x: 70.7106781186547,
                y: -70.7106781186547,
                angle: 45.0,
            },
            memory: Vec::new(),
        };

        let expectedpath = String::from(" M 0 0 L 70.71 -70.71");

        turtle_draw::step(input, '+', ANGLE_SHIFT);
        let new_path = turtle_draw::step(input, 'l', ANGLE_SHIFT);

        assert_eq!(new_path, expectedpath);
        assert!(input.current.angle - expected_output.current.angle < 1e-10);
        assert!(input.current.x - expected_output.current.x < 1e-10);
        assert!(input.current.y - expected_output.current.y < 1e-10);
    }

    #[test]
    fn minus_step() {
        let input = &mut types::DrawPosition {
            current: START_CURRENT,
            memory: Vec::new(),
        };

        let expected_output = types::DrawPosition {
            current: types::CurrentPosition {
                x: -70.7106781186547,
                y: -70.7106781186547,
                angle: -45.0,
            },
            memory: Vec::new(),
        };

        turtle_draw::step(input, '-', ANGLE_SHIFT);
        let new_path = turtle_draw::step(input, 'l', ANGLE_SHIFT);

        assert_eq!(new_path, String::from(" M 0 0 L -70.71 -70.71"));
        assert!(input.current.angle - expected_output.current.angle < 1e-10);
        assert!(input.current.x - expected_output.current.x < 1e-10);
        assert!(input.current.y - expected_output.current.y < 1e-10);
    }

    #[test]
    fn can_add_to_memory_step() {
        let input = &mut types::DrawPosition {
            current: START_CURRENT,
            memory: Vec::new(),
        };

        let expected_output = types::DrawPosition {
            current: types::CurrentPosition {
                x: 0.0,
                y: -100.0,
                angle: 0.0,
            },
            memory: Vec::new(),
        };

        turtle_draw::step(input, 'm', ANGLE_SHIFT);
        let new_path = turtle_draw::step(input, '[', ANGLE_SHIFT);

        let new_memory = &input.memory[0];

        assert_eq!(new_path, String::from(""));

        assert_eq!(new_memory.x, expected_output.current.x);
        assert_eq!(new_memory.y, expected_output.current.y);
        assert_eq!(new_memory.angle, expected_output.current.angle);
    }

    #[test]
    fn can_remove_memory_step() {
        let input = &mut types::DrawPosition {
            current: START_CURRENT,
            memory: Vec::new(),
        };

        let expected_output = types::DrawPosition {
            current: types::CurrentPosition {
                x: 0.0,
                y: -100.0,
                angle: 0.0,
            },
            memory: Vec::new(),
        };

        turtle_draw::step(input, 'm', ANGLE_SHIFT);
        turtle_draw::step(input, '[', ANGLE_SHIFT);
        turtle_draw::step(input, 'm', ANGLE_SHIFT);
        turtle_draw::step(input, 'm', ANGLE_SHIFT);
        turtle_draw::step(input, 'm', ANGLE_SHIFT);
        turtle_draw::step(input, ']', ANGLE_SHIFT);

        assert!(input.current.angle - expected_output.current.angle < 1e-10);
        assert!(input.current.x - expected_output.current.x < 1e-10);
        assert!(input.current.y - expected_output.current.y < 1e-10);
    }

    #[test]
    fn apply_turtle() {
        let new_path =
            turtle_draw::apply_turtle(String::from("m[+lm]l"), 0.0, 0.0, 0.0, ANGLE_SHIFT);

        assert_eq!(
            new_path,
            String::from(" M 0 -100 L 70.71 -170.71 M 0 -100 L 0 -200")
        );
    }
}
