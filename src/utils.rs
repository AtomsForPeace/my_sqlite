pub fn pad_or_truncate<const MAX_SIZE: usize>(
    input: &[u8],
    null_terminated: bool,
) -> [u8; MAX_SIZE] {
    let mut buffer = [0; MAX_SIZE];
    let max_len = if null_terminated {
        MAX_SIZE - 1
    } else {
        MAX_SIZE
    };
    let len = input.len().min(max_len);

    buffer[..len].copy_from_slice(&input[..len]);
    if null_terminated {
        buffer[len] = 0;
    }
    return buffer;
}
