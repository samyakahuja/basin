use super::BASE64_TABLE;

/// encode to base64 using standard character set.
pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    let input = input.as_ref();
    let input_len: usize = input.as_ref().len();

    let mut output_buf = vec![0; encode_size(input_len)];

    let incomplete_chunks_len = input_len % 3;
    let incomplete_chunks_index = input_len - incomplete_chunks_len;

    let mut input_index = 0;
    let mut output_index = 0;

    const LOW_SIX_BITS: u8 = 0x3F;

    // The final quantum of encoding input is an integral multiple of 24
    // bits; here, the final unit of encoded output will be an integral
    // multiple of 4 characters with no "=" padding.
    while input_index < incomplete_chunks_index {
        let input_chunk = &input[input_index..(input_index + 3)];
        let output_chunk = &mut output_buf[output_index..(output_index + 4)];

        output_chunk[0] = BASE64_TABLE[
            (input_chunk[0] >> 2) as usize
        ];
        output_chunk[1] = BASE64_TABLE[
            ((input_chunk[0] << 4 | input_chunk[1] >> 4) & LOW_SIX_BITS) as usize
        ];
        output_chunk[2] = BASE64_TABLE[
            ((input_chunk[1] << 2 | input_chunk[2] >> 6) & LOW_SIX_BITS) as usize
        ];
        output_chunk[3] = BASE64_TABLE[
            (input_chunk[2] & LOW_SIX_BITS) as usize
        ];

        input_index += 3;
        output_index += 4;
    }

    if incomplete_chunks_len == 1 {
        // The final quantum of encoding input is exactly 8 bits; here, the
        // final unit of encoded output will be two characters followed by
        // two "=" padding characters.
        output_buf[output_index] = BASE64_TABLE[
            (input[incomplete_chunks_index] >> 2) as usize
        ];
        output_buf[output_index + 1] = BASE64_TABLE[
            ((input[incomplete_chunks_index] << 4) & LOW_SIX_BITS) as usize
        ];
        output_buf[output_index + 2] = b'=';
        output_buf[output_index + 3] = b'=';
        output_index += 4;
    } else if incomplete_chunks_len == 2 {
        // The final quantum of encoding input is exactly 16 bits; here, the
        // final unit of encoded output will be three characters followed by
        // one "=" padding character.
        output_buf[output_index] = BASE64_TABLE[
            (input[incomplete_chunks_index] >> 2) as usize
        ];
        output_buf[output_index + 1] = BASE64_TABLE[
            ((input[incomplete_chunks_index] << 4 | input[incomplete_chunks_index + 1] >> 4) & LOW_SIX_BITS) as usize
        ];
        output_buf[output_index + 2] = BASE64_TABLE[
            ((input[incomplete_chunks_index + 1] << 2) & LOW_SIX_BITS) as usize
        ];
        output_buf[output_index + 3] = b'=';
        output_index += 4;
    }

    debug_assert_eq!(output_index, encode_size(input_len));

    String::from_utf8(output_buf).expect("Invalid utf-8")
}

/// number of n-bit words in encoded output
/// for base64 it would be number of 6-bit words
fn encode_size(input_size: usize) -> usize {
    const INPUT_GROUP_SIZE: usize = 3;
    const OUTPUT_GROUP_SIZE: usize = 4;

    let incomplete_groups = input_size % INPUT_GROUP_SIZE;
    let complete_groups = input_size / INPUT_GROUP_SIZE;
    let output_size = complete_groups * OUTPUT_GROUP_SIZE;

    if incomplete_groups > 0 {
        output_size + 4
    } else {
        output_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty() {
        assert_eq!("", encode(""));
    }

    #[test]
    fn encode_one_character() {
        assert_eq!("Zg==", encode("f"));
    }

    #[test]
    fn encode_two_character() {
        assert_eq!("Zm8=", encode("fo"));
    }

    #[test]
    fn encode_three_character() {
        assert_eq!("Zm9v", encode("foo"));
    }

    #[test]
    fn encode_four_character() {
        assert_eq!("Zm9vYg==", encode("foob"));
    }

    #[test]
    fn encode_five_character() {
        assert_eq!("Zm9vYmE=", encode("fooba"));
    }

    #[test]
    fn encode_six_character() {
        assert_eq!("Zm9vYmFy", encode("foobar"));
    }
}
