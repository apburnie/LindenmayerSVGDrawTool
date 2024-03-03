use crate::types;

// https://oreillymedia.github.io/Using_SVG/extras/ch08-precision.html
// Don't use numbers more than 2 digits after decimal and four digits before

const GROWTH_FACTOR: f64 = 100.0;

fn m_step(input: &mut types::DrawPosition) -> String {
    input.current.x += input.current.angle.to_radians().sin() * GROWTH_FACTOR;
    input.current.y -= input.current.angle.to_radians().cos() * GROWTH_FACTOR;

    return String::from("");
}

fn l_step(input: &mut types::DrawPosition) -> String {
    let inputx = input.current.x;
    let inputy = input.current.y;
    let newx = input.current.x + (input.current.angle.to_radians().sin() * GROWTH_FACTOR);
    let newy = input.current.y - (input.current.angle.to_radians().cos() * GROWTH_FACTOR);

    let displayx = (newx * 100.0).round() / 100.0;
    let displayy = (newy * 100.0).round() / 100.0;

    input.current.x = newx;
    input.current.y = newy;

    return format!(" M {inputx} {inputy} L {displayx} {displayy}");
}

fn plus_step(input: &mut types::DrawPosition, angle_shift: f64) -> String {
    input.current.angle += angle_shift;

    return String::from("");
}

fn minus_step(input: &mut types::DrawPosition, angle_shift: f64) -> String {
    input.current.angle -= angle_shift;

    return String::from("");
}

fn add_memory_step(input: &mut types::DrawPosition) -> String {
    input.memory.push(input.current.clone());

    return String::from("");
}

fn minus_memory_step(input: &mut types::DrawPosition) -> String {
    let result = input.memory.pop();

    match result {
        None => {}
        Some(v) => input.current = v,
    }

    return String::from("");
}

pub fn step(position: &mut types::DrawPosition, instruction: char, angle_shift: f64) -> String {
    if instruction == 'm' {
        return m_step(position);
    }

    if instruction == 'l' {
        return l_step(position);
    }

    if instruction == 'f' {
        return l_step(position);
    }

    if instruction == '+' {
        return plus_step(position, angle_shift);
    }

    if instruction == '-' {
        return minus_step(position, angle_shift);
    }

    if instruction == '[' {
        return add_memory_step(position);
    }

    if instruction == ']' {
        return minus_memory_step(position);
    }
    return String::from("");
}

pub fn apply_turtle(
    input: String,
    start_x: f64,
    start_y: f64,
    start_angle: f64,
    angle_shift: f64,
) -> String {
    let position = &mut types::DrawPosition {
        current: types::CurrentPosition {
            x: start_x,
            y: start_y,
            angle: start_angle,
        },
        memory: Vec::new(),
    };

    return input
        .chars()
        .map(|value| step(position, value, angle_shift))
        .collect();
}
