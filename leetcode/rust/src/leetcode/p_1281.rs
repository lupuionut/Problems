pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut rp = 1;
    let mut rs = 0;
    let mut r = n;
    while r != 0 {
        rp = rp * (r % 10);
        rs = rs + (r % 10);
        r = r / 10
    }
    return rp - rs;
}
