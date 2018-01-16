#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knothash() {
        assert_eq!(knothash("test"), "640d39586a768155a5aaaa40b5637281");
        assert_eq!(knothash("knothash"), "88a90984522e66079b09eb59f042e3c0");
        assert_eq!(knothash("longer string with spaces"), "d28baf60fdc7686c52846246eaea3268");
    }
}

fn knot(input:Vec<u8>, length: u8, skip_size: u8) -> Vec<u8> {
    let offset = length.wrapping_add(skip_size) as usize;
    let mut result = reverse(input, length as usize);
    result = shift(result, offset);
    result
}

fn shift(input: Vec<u8>, offset: usize) -> Vec<u8> {
    let mut result = input[offset..].to_vec();
    result.extend_from_slice(&input[..offset]);
    result
}

fn reverse(input: Vec<u8>, length: usize) -> Vec<u8> {
    let mut result = input[..length].to_vec();
    result.reverse();
    result.extend_from_slice(&input[length..]);
    result
}

pub fn hash(lengths: Vec<u8>, loops: u8) -> Vec<u8> {
    let mut hash: Vec<u8> = (0..256).map(|i| i as u8).collect();
    let mut skip_size = 0u8;
    let mut total_shifted = 0u8;
    for _ in 0..loops {
        for length in lengths.iter() {
            hash = knot(hash, *length, skip_size);
            total_shifted = total_shifted.wrapping_add(*length).wrapping_add(skip_size);
            skip_size = skip_size.wrapping_add(1);
        }
    }

    // shift back
    let offset = 0u8.wrapping_sub(total_shifted) as usize;
    shift(hash, offset)
}

fn convert_from_ascii(input: &str) -> Vec<u8> {
    input.lines().next().unwrap()
        .chars()
        .map(|item| item as u8)
        .collect()
}

pub fn knothash(input: &str) -> String {
    let mut lengths = convert_from_ascii(input);
    lengths.extend([17,31,73,47,23].iter().clone());
    let sparse_hash = hash(lengths, 64);

    let hex = sparse_hash.chunks(16)
        .map(|block| block.iter()
            .fold(0u8, |xor_result, element| xor_result ^ element))
        .map(|number| format!("{:02x}", number))
        .collect();

    return hex;
}
