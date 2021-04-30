pub struct f64Condition extends Condition{
minValue: f64
maxValue: f64
}
//#Test the data we’re interested in.
fn testValue() -> f64 {}

fn test() -> bool{
return minValue <= testValue <= maxValue
}