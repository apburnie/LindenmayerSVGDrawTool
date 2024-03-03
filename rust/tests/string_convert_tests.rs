#[cfg(test)]
mod tests {

    use fractal_draw::string_convert;
    use fractal_draw::types;

    const TESTSTRUCT: types::Rules = types::Rules {
        recursions: 10,
        axiom: 'm',
        start_position: types::StartPosition {
            x: 500,
            y: 1000,
            angle: 0,
            memory: types::Memory {
                x: Vec::new(),
                y: Vec::new(),
                angle: Vec::new(),
            },
        },
        string_convert: types::StringConvert {
            m: "l[-m]+m",
            l: "ll",
            f: "",
        },
    };

    // Test Case from https://en.wikipedia.org/wiki/L-system fractal (binary) tree [23 Jan 2024]

    #[test]
    fn step() {
        let result_one =
            string_convert::step(String::from(TESTSTRUCT.axiom), &TESTSTRUCT.string_convert);

        assert_eq!(result_one, "l[-m]+m");
        assert_eq!(
            string_convert::step(result_one, &TESTSTRUCT.string_convert),
            "ll[-l[-m]+m]+l[-m]+m"
        )
    }

    #[test]
    fn recursion() {
        let result_one = string_convert::recursion(
            &String::from(TESTSTRUCT.axiom),
            &TESTSTRUCT.string_convert,
            1,
        );

        assert_eq!(result_one, "l[-m]+m");

        let result_two = string_convert::recursion(
            &String::from(TESTSTRUCT.axiom),
            &TESTSTRUCT.string_convert,
            2,
        );

        assert_eq!(result_two, "ll[-l[-m]+m]+l[-m]+m");

        let result_three = string_convert::recursion(
            &String::from(TESTSTRUCT.axiom),
            &TESTSTRUCT.string_convert,
            3,
        );

        assert_eq!(
            result_three,
            "llll[-ll[-l[-m]+m]+l[-m]+m]+ll[-l[-m]+m]+l[-m]+m"
        )
    }
}
