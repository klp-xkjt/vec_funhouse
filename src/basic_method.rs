use crate::types::HowToKeep;
use crate::types::{RangeConfig, RangeMode, ConnectWithWhat};

/// 根据阈值和模式保留向量中的元素（单侧过滤）
///
/// 按照指定的模式，保留向量中大于/小于/大于等于/小于等于阈值的元素。
///
/// # 参数
///
/// * `vector` - 要过滤的可变向量引用
/// * `scope` - 阈值的引用（比较的基准值）
/// * `how_to_keep` - 过滤模式，决定保留哪一侧的元素
///
/// # 类型约束
///
/// * `T` - 必须实现 `PartialOrd`（可比较大小）
///
/// # 示例
///
/// ```
/// use vec_funhouse::{keep_value, HowToKeep};
///
/// let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
///
/// // 保留大于 5 的元素（注意传递引用）
/// keep_value(&mut v, &5, HowToKeep::Above);
/// assert_eq!(v, vec![6, 7, 8, 9]);
/// ```
///
/// # 支持的类型
///
/// 任何实现了 `PartialOrd` 的类型都可以使用，包括：
/// - 数值类型（`i32`、`f64` 等）
/// - 字符串（`String`、`&str`）
/// - 自定义结构体（只要实现了 `PartialOrd`）
///
/// # 注意
///
/// - `Above` 和 `AboveIncluding` 的区别在于是否包含阈值本身
/// - `Below` 和 `BelowIncluding` 同理
/// - 阈值以引用方式传递，调用后仍可继续使用
///
/// # 示例：使用非 Copy 类型
///
/// ```
/// # use vec_funhouse::{keep_value, HowToKeep};
/// let mut words = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
/// let threshold = "banana".to_string();
///
/// keep_value(&mut words, &threshold, HowToKeep::Below);
/// assert_eq!(words, vec!["apple".to_string()]);
/// // threshold 仍然可用，因为只是借用了引用
/// println!("阈值仍是: {}", threshold);
/// ```
pub fn keep_value<T>(vector: &mut Vec<T>, scope: &T, how_to_keep: HowToKeep)
where
    T: PartialOrd,
{
    match how_to_keep {
        HowToKeep::Above => vector.retain(|x| x > scope),
        HowToKeep::AboveIncluding => vector.retain(|x| x >= scope),
        HowToKeep::Below => vector.retain(|x| x < scope),
        HowToKeep::BelowIncluding => vector.retain(|x| x <= scope),
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
/// * `config` - 区间配置的引用（包含下限、上限、是否包含两端）
/// * `mode` - 过滤模式（保留区间内或保留区间外）
///
/// # 类型约束
///
/// * `T` - 必须实现 `PartialOrd`（可比较大小）
///
/// # 示例
///
/// ```
/// use vec_funhouse::{filter_range, RangeConfig, RangeMode};
///
/// let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let config = RangeConfig { low: 4, high: 7, include_low: true, include_high: true };
///
/// // 保留区间 [4, 7] 内的元素（注意传递引用）
/// filter_range(&mut v, &config, RangeMode::KeepInside);
/// assert_eq!(v, vec![4, 5, 6, 7]);
///
/// // config 仍然可用，因为只是借用了引用
/// println!("配置: {:?}", config);
/// ```
///
/// # 区间配置说明
///
/// 通过 `RangeConfig` 可以灵活定义区间边界：
/// - `include_low: true` → 下限包含（`≥ low`）
/// - `include_low: false` → 下限不包含（`> low`）
/// - `include_high: true` → 上限包含（`≤ high`）
/// - `include_high: false` → 上限不包含（`< high`）
///
/// # 注意
///
/// * `KeepInside` 保留满足区间条件的元素
/// * `KeepOutside` 保留区间外的元素（即删除区间内的元素）
/// * 区间配置支持包含/不包含边界，例如 `(4, 7]`、`[4, 7)` 等
/// * `config` 以引用方式传递，调用后仍可继续使用
///
/// # 示例：使用非 Copy 类型
///
/// ```
/// # use vec_funhouse::{filter_range, RangeConfig, RangeMode};
/// let mut words = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
/// let config = RangeConfig {
///     low: "apple".to_string(),
///     high: "cherry".to_string(),
///     include_low: true,
///     include_high: false,
/// };
///
/// filter_range(&mut words, &config, RangeMode::KeepInside);
/// // 保留 ["apple", "banana"]（因为 "cherry" 不包含）
/// assert_eq!(words, vec!["apple".to_string(), "banana".to_string()]);
/// ```
pub fn filter_range<T>(vector: &mut Vec<T>, config: &RangeConfig<T>, mode: RangeMode)
where
    T: PartialOrd,
{
    let low_op = |x: &T| {
        if config.include_low {
            x >= &config.low
        } else {
            x > &config.low
        }
    };
    let high_op = |x: &T| {
        if config.include_high {
            x <= &config.high
        } else {
            x < &config.high
        }
    };
    match mode {
        RangeMode::KeepInside => vector.retain(|x| low_op(x) && high_op(x)),
        RangeMode::KeepOutside => vector.retain(|x| !(low_op(x) && high_op(x)))
    };
}

/// 合并两个向量，将原向量的元素移动到目标向量
/// 
/// # 参数
/// - `vector1` - 第一个向量
/// - `vector2` - 第二个向量
/// - `which` - 选择将哪个向量移动到另一个
/// 
/// # 注意事项
/// 源向量被清空，元素所有权被移动到目标向量
/// 
/// # 实例
/// ```
/// use vec_funhouse::{ConnectWithWhat, connect_vectors};
/// let mut a = vec![1, 2];
/// let mut b = vec![3, 4];
/// connect_vectors(&mut a, &mut b, ConnectWithWhat::First);
/// assert_eq!(a, vec![1, 2, 3, 4]);
/// assert_eq!(b, vec![]);
/// ```
pub fn connect_vectors<T>(vector1: &mut Vec<T>, vector2: &mut Vec<T>, which: ConnectWithWhat) {
    match which {
        ConnectWithWhat::First => {
            vector1.extend(vector2.drain(..));
        },
        ConnectWithWhat::Second => {
            vector2.extend(vector1.drain(..));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_value1() {
        let mut vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        println!("原始: {:?}", vector);
        keep_value(&mut vector, &7, HowToKeep::BelowIncluding);
        println!("保留 ≤7: {:?}", vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn keep_value2() {
        let mut vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        println!("原始: {:?}", vector);
        keep_value(&mut vector, &6, HowToKeep::Above);
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
        filter_range(&mut vector, &config, RangeMode::KeepInside);
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
        filter_range(&mut vector, &config, RangeMode::KeepOutside);
        println!("排除 [4,6]: {:?}", vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 7, 8, 9]);
    }

    #[test]
    fn connect_vectors_test() {
        let mut a = vec![1, 2];
        let mut b = vec![3, 4];
        connect_vectors(&mut a, &mut b, ConnectWithWhat::First);
        assert_eq!(a, vec![1, 2, 3, 4]);
        assert_eq!(b, vec![]);
    }
}