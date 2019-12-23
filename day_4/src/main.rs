fn is_odd(elem: &[u8]) -> bool {
    let last_num = match elem.last() {
        Some(x) => x,
        None => panic!("No End"),
    };

    if (last_num % 2) != 0 {
        return true;
    } else {
        return false;
    }
}

fn not_decrease(elem: &[u8]) -> bool {
    let mut current_num = elem[0];

    for num in elem {
        if *num < current_num {
            return false;
        } else {
            current_num = *num;
        }
    }
    return true;
}

fn contains_duplicate(num: &String) -> bool {
    if num.contains("00") {
        return true;
    }
    if num.contains("11") {
        return true;
    }

    if num.contains("22") {
        return true;
    }

    if num.contains("33") {
        return true;
    }

    if num.contains("44") {
        return true;
    }

    if num.contains("55") {
        return true;
    }

    if num.contains("66") {
        return true;
    }

    if num.contains("77") {
        return true;
    }

    if num.contains("88") {
        return true;
    }

    if num.contains("99") {
        return true;
    }

    return false;
}

fn contains_double(sequence: &[u8]) -> bool {
    let mut padded_sequence = vec![0];
    for elem in sequence {
        padded_sequence.push(*elem);
    }
    padded_sequence.push(0);

    for i in 1..padded_sequence.len() - 1 {
        if (padded_sequence[i] == padded_sequence[i + 1])
            && (padded_sequence[i] != padded_sequence[i + 2])
            && (padded_sequence[i] != padded_sequence[i - 1])
        {
            return true;
        }
    }

    return false;
}

fn main() {
    let mut password_list = Vec::new();

    for i in 130254..=678275 {
        password_list.push(i.to_string());
    }

    password_list.retain(|x| not_decrease(x.as_bytes()));
    password_list.retain(|x| contains_duplicate(x));
    password_list.retain(|x| contains_double(x.as_bytes()));
    println!("{:?}", password_list.len());
}
