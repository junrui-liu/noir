// Tests a very simple program.
// 
// The features being tested is basic looping on brillig
fn main(sum: u32){
    assert(loop(4) == sum);
}

unconstrained fn loop(x: u32) -> u32 {
    let mut sum = 0;
    for i in 0..x {
        sum = sum + i;
    }
    sum
}
