static INPUT_MIN: u32 = 256310;
static INPUT_MAX: u32 = 732736;

#[derive(Debug)]
struct Password {
    repr: [u8; 6],
}

impl Password {
    fn get_u32(&self) -> u32 {
        let mut s = String::new();
        for digit in self.repr.iter() {
            s.push_str(&digit.to_string());
        }

        return s.parse::<u32>().unwrap();
    }

    fn from_u32(input: u32) -> Password {
        let mut repr: [u8; 6] = [0; 6];
        for (idx, c) in input.to_string().char_indices() {
            repr[idx] = c.to_digit(10).unwrap() as u8;
        }

        return Password { repr };
    }
}

pub fn start() {
    let mut possible_passwords: Vec<u32> = Vec::new();
    let mut password = Password::from_u32(INPUT_MIN);

    loop {
        if only_ascending_digits(&password) && adjacent_double_digits(&password) {
            possible_passwords.push(password.get_u32());
        }

        password = get_next_password(&password);

        if password.get_u32() > INPUT_MAX {
            break;
        }
    }

    println!("Number of possible passwords: {}", possible_passwords.len());
}

fn only_ascending_digits(password: &Password) -> bool {
    for idx in 0..password.repr.len() - 1 {
        if password.repr[idx] > password.repr[idx + 1] {
            return false;
        }
    }
    return true;
}

fn adjacent_double_digits(password: &Password) -> bool {
    for idx in 0..password.repr.len() - 1 {
        if password.repr[idx] == password.repr[idx + 1] {
            if idx == 0 {
                if password.repr[idx] != password.repr[idx + 2] {
                    return true;
                }
                continue;
            }

            if idx + 2 >= password.repr.len() {
                if password.repr[idx] != password.repr[idx - 1] {
                    return true;
                }
                continue;
            }

            if password.repr[idx] != password.repr[idx - 1]
                && password.repr[idx] != password.repr[idx + 2]
            {
                return true;
            }
        }
    }
    return false;
}

fn get_next_password(password: &Password) -> Password {
    let mut next = Password::from_u32(password.get_u32() + 1);
    for idx in 0..next.repr.len() - 1 {
        while next.repr[idx] > next.repr[idx + 1] {
            next.repr[idx + 1] += 1 as u8;
        }
    }
    return next;
}
