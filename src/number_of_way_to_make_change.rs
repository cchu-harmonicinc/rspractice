#[cfg(test)]
pub fn solution(target: u32, denoms: Vec<u32>) -> u32 {
    // create a table of size target+1
    // return the result which store in the table[target]
    let mut table= vec![0; (target as usize) + 1];
    // initializing the base cases
    table[0] = 1;
    // construct the dp table iteratively
    for &d in denoms.iter() {
        for i in d..target+1 {
            table[i as usize] += table[(i as usize) - (d as usize)];
        }
    }
    table[(target as usize)]
}