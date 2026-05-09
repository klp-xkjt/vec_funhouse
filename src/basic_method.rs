use crate::types::HowToKeep;
use crate::types::{RangeConfig, RangeMode};

/// 根据阈值和模式保留向量中的元素（单侧过滤）
///
/// 按照指定的模式，保留向量中大于/小于/大于等于/小于等于阈值的元素。
///
/// # 参数
///
/// * `vector` - 要过滤的可变向量引用
/// * `scope` - 阈值（比较的基准值）
/// * `how_to_keep` - 过滤模式，决定保留哪一侧的元素
///
/// # 类型约束
///
/// * `T` - 必须实现 `PartialOrd`（可比较大小）和 `Copy`（可复制）
///
/// # 示例
///
/// ```
/// use vec_funhouse::{keep_value, HowToKeep};
///
/// let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
///
/// // 保留大于 5 的元素
/// keep_value(&mut v, 5, HowToKeep::Above);
/// assert_eq!(v, vec![6, 7, 8, 9]);
/// ```
///
/// # 注意
///
/// `Above` 和 `AboveIncluding` 的区别在于是否包含阈值本身。
/// `Below` 和 `BelowIncluding` 同理。
pub fn keep_value<T>(vector: &mut Vec<T>, scope: T, how_to_keep: HowToKeep)
where
    T: PartialOrd + Copy,
{
    match &how_to_keep {
        HowToKeep::Above => vector.retain(|x| *x > scope),
        HowToKeep::AboveIncluding => vector.retain(|x| *x >= scope),
        HowToKeep::Below => vector.retain(|x| *x < scope),
        HowToKeep::BelowIncluding => vector.retain(|x| *x <= scope),
    }
}

/// 根据区间配置和模式过滤向量中的元素（区间过滤）
///
/// 按照指定的区间配置（上下限及是否包含边界）和过滤模式，
/// 保留区间内或区间外的元素。
///
/// # 参数
///
/// * `vector` - 要过滤的可变向量引用
/// * `config` - 区间配置（包含下限、上限、是否包含两端）
/// * `mode` - 过滤模式（保留区间内或保留区间外）
///
/// # 类型约束
///
/// * `T` - 必须实现 `PartialOrd`（可比较大小）和 `Copy`（可复制）
///
/// # 示例
///
/// ```
/// use vec_funhouse::{filter_range, RangeConfig, RangeMode};
///
/// let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let config = RangeConfig { low: 4, high: 7, include_low: true, include_high: true };
///
/// // 保留区间 [4, 7] 内的元素
/// filter_range(&mut v, config, RangeMode::KeepInside);
/// assert_eq!(v, vec![4, 5, 6, 7]);
/// ```
///
/// # 注意
///
/// * `KeepInside` 保留满足 `low ≤ x ≤ high`（根据边界配置可能不含等号）的元素
/// * `KeepOutside` 保留区间外的元素（即删除区间内的元素）
/// * 区间配置支持包含/不包含边界，例如 `(4, 7]`、`[4, 7)` 等
pub fn filter_range<T>(vector: &mut Vec<T>, config: RangeConfig<T>, mode: RangeMode)
where
    T: PartialOrd + Copy,
{
    let low_op = |x: T| {
        if config.include_low {
            x >= config.low
        } else {
            x > config.low
        }
    };
    let high_op = |x: T| {
        if config.include_high {
            x <= config.high
        } else {
            x < config.high
        }
    };
    match mode {
        RangeMode::KeepInside => vector.retain(|x| low_op(*x) && high_op(*x)),
        RangeMode::KeepOutside => vector.retain(|x| !(low_op(*x) && high_op(*x)))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_value1() {
        let mut vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        println!("原始: {:?}", vector);
        keep_value(&mut vector, 7, HowToKeep::BelowIncluding);
        println!("保留 ≤7: {:?}", vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn keep_value2() {
        let mut vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        println!("原始: {:?}", vector);
        keep_value(&mut vector, 6, HowToKeep::Above);
        println!("保留 >6: {:?}", vector);
        assert_eq!(vector, vec![7, 8, 9]);
    }

    #[test]
    fn filter_range1() {
        let config = RangeConfig { 
            low: 4,
            high: 6,
            include_high: true,
            include_low: true
        };

        let mut vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        println!("原始: {:?}", vector);
        filter_range(&mut vector, config, RangeMode::KeepInside);
        println!("保留 [4,6]: {:?}", vector);
        assert_eq!(vector, vec![4, 5, 6]);
    }

    #[test]
    fn filter_range2() {
        let config = RangeConfig { 
            low: 4,
            high: 6,
            include_high: true,
            include_low: true
        };

        let mut vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        println!("原始: {:?}", vector);
        filter_range(&mut vector, config, RangeMode::KeepOutside);
        println!("排除 [4,6]: {:?}", vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 7, 8, 9]);
    }
}