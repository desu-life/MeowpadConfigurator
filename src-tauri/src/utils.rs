pub fn compare_version(version1: &str, version2: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;

    // 检查版本号的格式是否正确
    let re = regex::Regex::new(r"^\d+(\.\d+)*$").unwrap();
    if !re.is_match(version1) || !re.is_match(version2) {
        return Equal
    }

    // 将版本号转换为数字向量
    let v1: Vec<u64> = version1.split('.').map(|s| s.parse().unwrap()).collect();
    let v2: Vec<u64> = version2.split('.').map(|s| s.parse().unwrap()).collect();

    // 比较数字向量
    for i in 0..std::cmp::max(v1.len(), v2.len()) {
        let n1 = v1.get(i).unwrap_or(&0);
        let n2 = v2.get(i).unwrap_or(&0);
        match n1.cmp(n2) {
            Greater => return Greater,
            Less => return Less,
            Equal => (),
        }
    }

    // 版本号相等
    Equal
}

