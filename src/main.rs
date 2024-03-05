use std::collections::HashMap;

#[allow(unused_variables)]

fn encrypt_caesar(input: &str, shift: u8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = (c as u8).wrapping_sub(base).wrapping_add(shift).wrapping_rem(26) + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn decrypt_caesar(input: &str, shift: u8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = (c as u8).wrapping_sub(base).wrapping_sub(shift).wrapping_rem(26) + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn encrypt_rail_fence(plain_text: &str, num_rails: usize) -> String {
    if num_rails == 1 {
        return plain_text.to_string();
    }

    let mut rails = vec![Vec::new(); num_rails];
    let mut rail = 0;
    let mut direction = 1;

    for char in plain_text.chars() {
        rails[rail].push(char);
        if rail == 0 {
            direction = 1;
        } else if rail == num_rails - 1 {
            direction = -1;
        }
        rail = ((rail as isize) + direction) as usize;
    }

    rails.into_iter().flatten().collect()
}

fn decrypt_rail_fence(cipher_text: &str, num_rails: usize) -> String {
    if num_rails == 1 {
        return cipher_text.to_string();
    }

    let mut rail_lengths = vec![0; num_rails];
    let mut rail = 0;
    let mut direction = 1;

    for _ in cipher_text.chars() {
        rail_lengths[rail] += 1;
        if rail == 0 {
            direction = 1;
        } else if rail == num_rails - 1 {
            direction = -1;
        }
        rail = ((rail as isize) + direction) as usize;
    }

    let mut rails = vec![Vec::new(); num_rails];
    let mut index = 0;
    for (rail, &length) in rail_lengths.iter().enumerate() {
        for _ in 0..length {
            if let Some(character) = cipher_text.chars().nth(index) {
                rails[rail].push(character);
                index += 1;
            }
        }
    }

    let mut result = String::new();
    let mut idxs = vec![0; num_rails];
    rail = 0;
    direction = 1;

    for _ in 0..cipher_text.len() {
        result.push(rails[rail][idxs[rail]]);
        idxs[rail] += 1;

        if rail == 0 {
            direction = 1;
        } else if rail == num_rails - 1 {
            direction = -1;
        }
        rail = ((rail as isize) + direction) as usize;
    }

    result
}

fn encrypt_columnar(plain_text: &str, key: &str) -> String {
    let len_key = key.len();
    let len_plain = plain_text.len();
    let row = (len_plain + len_key - 1) / len_key;

    let mut matrix = vec![vec![' '; len_key]; row];

    for (i, ch) in plain_text.chars().enumerate() {
        let r = i / len_key;
        let c = i % len_key;
        matrix[r][c] = ch;
    }

    let mut key_indices: Vec<_> = key.chars().enumerate().collect();
    key_indices.sort_unstable_by_key(|&(_, ch)| ch);

    let mut cipher_text = String::new();
    for &(index, _) in &key_indices {
        for row in &matrix {
            cipher_text.push(row[index]);
        }
    }

    cipher_text
}

fn decrypt_columnar(encrypted_text: &str, key: &str) -> String {
    let len_key = key.len();
    let len_cipher = encrypted_text.len();
    let rows = (len_cipher + len_key - 1) / len_key;

    let mut key_indices: Vec<_> = key.chars().enumerate().collect();
    key_indices.sort_unstable_by_key(|&(_, ch)| ch);

    let mut col_lengths = vec![rows; len_key];
    for i in 0..(len_cipher % len_key) {
        col_lengths[i] -= 1;
    }

    let mut col_order: HashMap<char, usize> = HashMap::new();
    for (_, &(index, ch)) in key_indices.iter().enumerate() {
        col_order.insert(ch, index);
    }

    let mut matrix = vec![vec![' '; len_key]; rows];
    let mut idx = 0;
    for &(_, col_ch) in &key_indices {
        if let Some(&col) = col_order.get(&col_ch) {
            for r in 0..col_lengths[col] {
                if idx < len_cipher {
                    matrix[r][col] = encrypted_text.chars().nth(idx).unwrap();
                    idx += 1;
                }
            }
        }
    }

    let mut p_text = String::new();
    for r in 0..rows {
        for c in 0..len_key {
            p_text.push(matrix[r][c]);
        }
    }

    p_text.trim_end().to_string()
}


fn main() {
    let message = "Salam, Man Adib Nikjou Hastam";
    let shift = 3; // The shift value for the Caesar cipher
    let encrypted_caesar = encrypt_caesar(message, shift);
    println!("Encrypted Caesar: {}", encrypted_caesar);

    let decrypted_caesar = decrypt_caesar(&encrypted_caesar, shift);
    println!("Decrypted Caesar: {}", decrypted_caesar);

    let encrypted_rail_fence = encrypt_rail_fence(&decrypted_caesar, 3);
    println!("Encrypted Rail Fence: {}", encrypted_rail_fence);

    let decrypted_rail_fence = decrypt_rail_fence(&encrypted_rail_fence, 3);
    println!("Decrypted Rail Fence: {}", decrypted_rail_fence);

    let encrypted_coloumnar = encrypt_columnar(&decrypted_rail_fence, "14523");
    println!("Encrypted coloumnar: {}", encrypted_coloumnar);

    let decrypted_coloumnar = decrypt_columnar(&encrypted_coloumnar, "14523");
    println!("Decrypted coloumnar: {}", decrypted_coloumnar);
}
