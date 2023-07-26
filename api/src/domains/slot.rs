use std::collections::HashSet;

use chrono::{Duration, NaiveDateTime};
use derive_new::new;
use itertools::Itertools;

use super::slot_range::SlotRange;

/// 30分の時間枠
#[derive(Debug, Clone, new, PartialEq)]
pub struct Slot {
    pub start_date: NaiveDateTime,
}

impl Slot {
    // 1枠は30分
    pub fn duration() -> chrono::Duration {
        chrono::Duration::minutes(30)
    }
    pub fn end_date(&self) -> chrono::NaiveDateTime {
        self.start_date + Self::duration()
    }

    // 時間が連続しているかどうか
    pub fn is_continuous(&self, other: &Self) -> bool {
        let diff = self.end_date() - other.start_date;
        diff == Duration::zero()
    }
}

/// 例えば、10:00-10:30, 10:30-11:00, 11:00-11:30, 12:00-12:30のような時間帯を
/// 10:00-11:30, 12:00-12:30のようにまとめる
pub fn collect_slot_ranges(times: &[Slot]) -> Vec<SlotRange> {
    if times.is_empty() {
        panic!("times must not be empty");
    }
    if times.len() == 1 {
        return vec![SlotRange::from(times[0].clone())];
    }
    // order by times.start_date
    let sorted = times.iter().sorted_by_key(|t| t.start_date).collect_vec();

    let mut durations = vec![];
    let mut compined_index_set = HashSet::new();
    compined_index_set.insert(0);
    for i in 0..sorted.len() - 1 {
        if !sorted[i].is_continuous(sorted[i + 1]) {
            combine_slots(&mut compined_index_set, &sorted, &mut durations);
        }
        compined_index_set.insert(i + 1);
    }
    combine_slots(&mut compined_index_set, &sorted, &mut durations);
    durations
}

fn combine_slots(
    compined_index_set: &mut HashSet<usize>,
    sorted: &[&Slot],
    durations: &mut Vec<SlotRange>,
) {
    if !compined_index_set.is_empty() {
        let start_index = *compined_index_set.iter().min().unwrap();
        let end_index = *compined_index_set.iter().max().unwrap();
        let du = SlotRange::new(sorted[start_index].start_date, sorted[end_index].end_date());
        durations.push(du);
        compined_index_set.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_combined() {
        let slot1 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        let slot2 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 00:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );

        assert!(slot1.is_continuous(&slot2));
        assert!(!slot2.is_continuous(&slot1));
        let slot3 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 01:15:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        assert!(!slot1.is_continuous(&slot3));
        assert!(!slot2.is_continuous(&slot3));
    }
    #[test]
    fn test_collect_slot_ranges1() {
        // test collect_slot_ranges method
        let slot1 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        let slot2 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        let slot3 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        let slot4 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );

        let ranges = collect_slot_ranges(vec![slot1, slot2, slot3, slot4].as_slice());
        assert_eq!(ranges.len(), 2);
        assert_eq!(
            ranges[0],
            SlotRange::new(
                NaiveDateTime::parse_from_str("2020-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2020-01-01 11:30:00", "%Y-%m-%d %H:%M:%S").unwrap()
            )
        );
        assert_eq!(
            ranges[1],
            SlotRange::new(
                NaiveDateTime::parse_from_str("2020-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2020-01-01 12:30:00", "%Y-%m-%d %H:%M:%S").unwrap()
            )
        );
    }
    #[test]
    fn test_collect_slot_ranges2() {
        let slot1 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        let slot2 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        let ranges = collect_slot_ranges(vec![slot1, slot2].as_slice());
        assert_eq!(ranges.len(), 1);
        assert_eq!(
            ranges[0],
            SlotRange::new(
                NaiveDateTime::parse_from_str("2020-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2020-01-01 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
            )
        );
    }
    #[test]
    fn test_collect_slot_ranges3() {
        let slot1 = Slot::new(
            NaiveDateTime::parse_from_str("2020-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );

        let ranges = collect_slot_ranges(vec![slot1].as_slice());
        assert_eq!(ranges.len(), 1);
        assert_eq!(
            ranges[0],
            SlotRange::new(
                NaiveDateTime::parse_from_str("2020-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                NaiveDateTime::parse_from_str("2020-01-01 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap()
            )
        );
    }
}
