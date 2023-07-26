use derive_new::new;

use super::slot::Slot;

#[derive(Debug, Clone, new, PartialEq)]
pub struct SlotRange {
    // slotrange
    pub start: chrono::NaiveDateTime,
    pub end: chrono::NaiveDateTime,
}

impl From<Slot> for SlotRange {
    fn from(time: Slot) -> Self {
        Self {
            start: time.start_date,
            end: time.end_date(),
        }
    }
}
impl SlotRange {
    /// 重なる時間があるかどうか
    fn intersects(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
            || other.start <= self.end && self.start <= other.end
    }
    /// 重なる時間がある場合、重なる時間の範囲を返す
    pub fn intersect_slot_range(&self, other: &Self) -> Option<SlotRange> {
        if !self.intersects(other) {
            return None;
        }
        let start = if self.start > other.start {
            self.start
        } else {
            other.start
        };
        let end = if self.end < other.end {
            self.end
        } else {
            other.end
        };

        Some(SlotRange::new(start, end))
    }
    pub fn to_slots(&self) -> Vec<Slot> {
        let start = self.start;
        let end = self.end;
        let mut slots = vec![];
        let mut current = start;
        while current + Slot::duration() <= end {
            slots.push(Slot::new(current));
            current += Slot::duration();
        }
        slots
    }
}

fn intersect_slot_ranges(lhs: &[SlotRange], rhs: &[SlotRange]) -> Vec<SlotRange> {
    let mut slot_ranges: Vec<SlotRange> = vec![];
    for l in lhs {
        for r in rhs {
            let Some(range) = l.intersect_slot_range(r) else {
                continue;
            };
            slot_ranges.push(range);
        }
    }
    slot_ranges
}

/// 受け取ったSlotRangeの集合の内、重なっている時間帯を返す
pub fn intersect_slot_ranges_array(ranges: Vec<Vec<SlotRange>>) -> Vec<SlotRange> {
    if ranges.is_empty() {
        return vec![];
    }
    let first = &ranges[0];
    ranges
        .iter()
        .skip(1)
        .fold(first.clone(), |acc, slot_ranges| {
            intersect_slot_ranges(&acc, slot_ranges)
        })
}
#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDateTime;

    fn to_date(date: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap()
    }

    #[test]
    fn test_to_slot() {
        let slot_range = SlotRange::new(
            to_date("2020-01-01 10:00:00"),
            to_date("2020-01-01 11:30:00"),
        );
        let slots = slot_range.to_slots();
        assert_eq!(
            slots,
            vec![
                Slot::new(to_date("2020-01-01 10:00:00")),
                Slot::new(to_date("2020-01-01 10:30:00")),
                Slot::new(to_date("2020-01-01 11:00:00")),
            ]
        );
    }
    #[test]
    fn test_intersect_slot_range_array() {
        let ret = intersect_slot_ranges_array(vec![]);
        assert_eq!(ret, vec![]);

        let slot_ranges1 = vec![
            SlotRange::new(
                to_date("2020-01-01 10:00:00"),
                to_date("2020-01-01 12:30:00"),
            ),
            SlotRange::new(
                to_date("2020-01-02 12:30:00"),
                to_date("2020-01-03 14:30:00"),
            ),
        ];
        let slot_ranges2 = vec![
            SlotRange::new(
                to_date("2020-01-01 11:00:00"),
                to_date("2020-01-01 12:30:00"),
            ),
            SlotRange::new(
                to_date("2020-01-01 18:30:00"),
                to_date("2020-01-03 11:30:00"),
            ),
        ];
        let ret = intersect_slot_ranges_array(vec![slot_ranges1, slot_ranges2]);
        assert_eq!(
            ret,
            vec![
                SlotRange::new(
                    to_date("2020-01-01 11:00:00"),
                    to_date("2020-01-01 12:30:00"),
                ),
                SlotRange::new(
                    to_date("2020-01-02 12:30:00"),
                    to_date("2020-01-03 11:30:00"),
                ),
            ]
        );

        let slot_ranges1 = vec![
            SlotRange::new(
                to_date("2020-01-01 10:00:00"),
                to_date("2020-01-01 12:30:00"),
            ),
            SlotRange::new(
                to_date("2020-01-01 12:30:00"),
                to_date("2020-01-01 11:30:00"),
            ),
        ];
        let slot_ranges2 = vec![
            SlotRange::new(
                to_date("2020-01-02 10:00:00"),
                to_date("2020-01-02 12:30:00"),
            ),
            SlotRange::new(
                to_date("2021-01-02 12:30:00"),
                to_date("2023-01-03 11:30:00"),
            ),
        ];
        let ret = intersect_slot_ranges_array(vec![slot_ranges1, slot_ranges2]);
        assert_eq!(ret, vec![]);
    }
    #[test]
    fn test_intersect_slot_range() {
        let slot_ranges1 = vec![
            SlotRange::new(
                to_date("2020-01-01 10:00:00"),
                to_date("2020-01-01 12:30:00"),
            ),
            SlotRange::new(
                to_date("2020-01-01 14:30:00"),
                to_date("2020-01-01 16:30:00"),
            ),
        ];
        let slot_ranges2 = vec![
            SlotRange::new(
                to_date("2020-01-01 11:15:00"),
                to_date("2020-01-01 13:00:00"),
            ),
            SlotRange::new(
                to_date("2020-01-01 16:15:00"),
                to_date("2020-01-01 17:30:00"),
            ),
        ];

        let intersected = intersect_slot_ranges(&slot_ranges1, &slot_ranges2);
        assert_eq!(
            intersected,
            vec![
                SlotRange::new(
                    to_date("2020-01-01 11:15:00"),
                    to_date("2020-01-01 12:30:00"),
                ),
                SlotRange::new(
                    to_date("2020-01-01 16:15:00"),
                    to_date("2020-01-01 16:30:00"),
                ),
            ]
        );
    }
    #[test]
    fn test_intersect() {
        assert!(SlotRange::new(
            to_date("2020-01-01 10:00:00"),
            to_date("2020-01-01 10:30:00")
        )
        .intersects(&SlotRange::new(
            to_date("2020-01-01 09:45:00"),
            to_date("2020-01-01 10:15:00")
        )));

        assert!(SlotRange::new(
            to_date("2020-01-01 10:00:00"),
            to_date("2020-01-01 10:30:00")
        )
        .intersects(&SlotRange::new(
            to_date("2020-01-01 10:15:00"),
            to_date("2020-01-01 10:45:00")
        )));

        // 年が違うので交差しない
        assert!(!SlotRange::new(
            to_date("2020-01-01 10:00:00"),
            to_date("2020-01-01 10:30:00")
        )
        .intersects(&SlotRange::new(
            to_date("2021-01-01 10:15:00"),
            to_date("2021-01-01 10:45:00")
        )));
    }
}
