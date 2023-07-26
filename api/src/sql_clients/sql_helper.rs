/// 指定した個数の?を文字列で作成する。例えば3を指定すると、?,?,?を返却する
/// - count: ほしい?の数
pub fn create_place_holder(count: usize) -> String {
    (0..count).map(|_| "?").collect::<Vec<_>>().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_place_holder() {
        assert_eq!(create_place_holder(3), "?,?,?");
    }
}
