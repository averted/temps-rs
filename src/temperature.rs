#[derive(Debug, PartialEq)]
pub enum Temperature {
    C(f32),
    F(f32),
}

impl Temperature {
    pub fn convert(&self) -> Temperature {
        match self {
            Temperature::C(val) => {
                let result = val * (9.0 / 5.0) + 32.0;
                Temperature::F((result * 100.0).round() / 100.0)
            }
            Temperature::F(val) => {
                let result = (val - 32.0) * 5.0 / 9.0;
                Temperature::C((result * 100.0).round() / 100.0)
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Temperature::C(val) => format!("{} C", val),
            Temperature::F(val) => format!("{} F", val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_convertion_works_correctly() {
        assert_eq!(Temperature::F(3.0).convert(), Temperature::C(-16.11));
        assert_eq!(Temperature::F(30.0).convert(), Temperature::C(-1.11));

        assert_eq!(Temperature::C(3.0).convert(), Temperature::F(37.4));
        assert_eq!(Temperature::C(30.0).convert(), Temperature::F(86.0));
    }
}
