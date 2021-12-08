use super::Digit;
use super::SegmentDigit;
use itertools::Itertools;
use std::collections::HashMap;

#[allow(unstable_name_collisions)]
pub fn get_sum_output_signals(values: Vec<([String; 10], [String; 4])>) -> i64 {
    let len_to_digit = get_len_to_digit_map();

    let mut total = 0i64;
    for (input_signals, output_signals) in values {
        let lsegments: Vec<SegmentDigit> = input_signals
            .into_iter()
            .map(|x| SegmentDigit::new(x, &len_to_digit))
            .collect();

        let mapping = determine_mappings(lsegments);

        let value: String = output_signals
            .into_iter()
            .map(|x| mapping.get(&x).unwrap())
            .cloned()
            .map(|d| Digit::get_value(d).to_string())
            .intersperse("".to_string())
            .collect();

        total += value.parse::<i64>().unwrap();
    }

    total
}

fn determine_mappings(input_signals: Vec<SegmentDigit>) -> HashMap<String, Digit> {
    let mut determined: HashMap<Digit, SegmentDigit> = HashMap::new();
    for sd in input_signals.iter().filter(|sd| sd.is_determined()) {
        let d = sd.value.unwrap();
        determined.insert(d, sd.clone());
    }

    let len_6: Vec<SegmentDigit> = input_signals
        .iter()
        .filter(|sd| sd.len() == 6usize)
        .cloned()
        .collect();

    for mut sd in len_6 {
        if sd.get_intersection_count(determined.get(&Digit::One).unwrap()) == 1 {
            sd.determine(Digit::Six);
        } else if sd.get_intersection_count(determined.get(&Digit::Four).unwrap()) == 4 {
            sd.determine(Digit::Nine);
        } else {
            sd.determine(Digit::Zero);
        }
        determined.insert(sd.value.unwrap(), sd);
    }

    let len_5: Vec<SegmentDigit> = input_signals
        .iter()
        .filter(|sd| sd.len() == 5usize)
        .cloned()
        .collect();

    for mut sd in len_5 {
        if sd.get_intersection_count(determined.get(&Digit::One).unwrap()) == 2 {
            sd.determine(Digit::Three);
        } else if sd.get_intersection_count(determined.get(&Digit::Four).unwrap()) == 3 {
            sd.determine(Digit::Five);
        } else {
            sd.determine(Digit::Two);
        }
        determined.insert(sd.value.unwrap(), sd);
    }

    let mut mappings: HashMap<String, Digit> = HashMap::new();
    for (d, sd) in determined {
        mappings.insert(sd.wires, d);
    }
    mappings
}

fn get_len_to_digit_map() -> HashMap<usize, Vec<Digit>> {
    let mut len_to_digit: HashMap<usize, Vec<Digit>> = HashMap::new();
    for digit in Digit::VALUES.iter().copied() {
        let chars = Digit::get_chars(digit);
        len_to_digit
            .entry(chars.len())
            .or_insert_with(Vec::new)
            .push(digit);
    }
    len_to_digit
}
