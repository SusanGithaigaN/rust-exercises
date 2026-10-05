#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bmi_category() {
        assert_eq!(bmi_category(18.0, 1.0), "underweight");
        assert_eq!(bmi_category(18.5, 1.0), "normal");
        assert_eq!(bmi_category(25.0, 1.0), "overweight");
        assert_eq!(bmi_category(30.0, 1.0), "obese");
    }

    #[test]
    fn check_describe_temp() {
        assert_eq!(describe_temp(-1.0), "freezing");
        assert_eq!(describe_temp(0.0), "cold");
        assert_eq!(describe_temp(15.0), "warm");
        assert_eq!(describe_temp(25.0), "hot");
    }
}
