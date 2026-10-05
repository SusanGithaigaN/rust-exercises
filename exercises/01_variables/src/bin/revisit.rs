// TOPIC 01 — Variables, Mutability, Functions
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing recall/extend.
// Write everything from scratch — do not look at your previous files.
//
// TASK A: fn bmi_category(weight_kg: f64, height_m: f64) -> &'static str
//   BMI = weight_kg / (height_m * height_m)
//   "underweight"  if BMI < 18.5
//   "normal"       if 18.5 <= BMI < 25.0
//   "overweight"   if 25.0 <= BMI < 30.0
//   "obese"        if BMI >= 30.0
//
// TASK B: fn describe_temp(celsius: f64) -> &'static str
//   "freezing"  if celsius < 0.0
//   "cold"      if 0.0 <= celsius < 15.0
//   "warm"      if 15.0 <= celsius < 25.0
//   "hot"       if celsius >= 25.0
//
// Expected output:
//   70kg, 1.75m: normal
//   50kg, 1.75m: underweight
//   100kg, 1.75m: obese
//   -5°C: freezing
//   10°C: cold
//   30°C: hot
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement bmi_category and describe_temp above main, then call here
}
