#[cfg(test)]
pub fn solution(v: Vec<u32>) -> u32 {
    if v.len() == 0 { return 0; }

    let n = v.len();

    let mut table = vec![0; n + 1];

    // base case
    table[1] = v[0];
    for (index, &value) in v.iter().enumerate().skip(1) {
        if table[index] != table[index-1] {
            table[index+1] = if value + table[index-1] > table[index] {
                value + table[index-1]
            } else {
                table[index]
            }
        } else {
            table[index+1] = table[index] + value;
        }
    }
    table[n]
}