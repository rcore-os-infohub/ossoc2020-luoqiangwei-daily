// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct<'a> {
    // TODO: Something goes here
    name: &'a str,
    hex: &'a str
}

struct ColorTupleStruct<'a>(/* TODO: Something goes here */&'a str, &'a str);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =
        let green = ColorClassicStruct {
            name: "green", 
            hex: "#00FF00"
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // let green =
        let green = ColorTupleStruct("green", "#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        // let unit_struct =
        let unit_struct = UnitStruct{};
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}