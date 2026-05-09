/// 单侧过滤模式
///
/// 用于 `keep_value` 函数，指定保留阈值哪一侧的元素。
///
/// # 示例
///
/// ```
/// use vec_funhouse::{keep_value, HowToKeep};
///
/// let mut v = vec![1, 2, 3, 4, 5];
/// keep_value(&mut v, 3, HowToKeep::Above);      // 保留 >3  → [4, 5]
/// keep_value(&mut v, 3, HowToKeep::Below);      // 保留 <3  → [1, 2]
/// keep_value(&mut v, 3, HowToKeep::AboveIncluding); // 保留 ≥3 → [3, 4, 5]
/// keep_value(&mut v, 3, HowToKeep::BelowIncluding); // 保留 ≤3 → [1, 2, 3]
/// ```
pub enum HowToKeep {
    /// 保留大于阈值的元素（不包含边界）`>`
    Above,
    /// 保留大于等于阈值的元素（包含边界）`>=`
    AboveIncluding,
    /// 保留小于阈值的元素（不包含边界）`<`
    Below,
    /// 保留小于等于阈值的元素（包含边界）`<=`
    BelowIncluding,
}

/// 区间过滤模式
///
/// 用于 `filter_range` 函数，指定保留区间内还是区间外的元素。
///
/// # 示例
///
/// ```
/// use vec_funhouse::{filter_range, RangeConfig, RangeMode};
///
/// let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let config = RangeConfig { low: 4, high: 7, include_low: true, include_high: true };
///
/// filter_range(&mut v, config, RangeMode::KeepInside);  // 保留 [4,5,6,7]
/// filter_range(&mut v, config, RangeMode::KeepOutside); // 排除 [4,5,6,7]
/// ```
#[derive(Copy, Clone)]
pub enum RangeMode {
    /// 保留区间内的元素
    KeepInside,
    /// 保留区间外的元素（即删除区间内的元素）
    KeepOutside,
}

/// 区间配置
///
/// 定义一个数值区间，以及两端是否包含边界值。
///
///
/// # 示例
///
/// ```
/// use vec_funhouse::RangeConfig;
///
/// // 区间 [4, 7]（包含两端）
/// let config1 = RangeConfig { low: 4, high: 7, include_low: true, include_high: true };
///
/// // 区间 (4, 7)（不包含两端）
/// let config2 = RangeConfig { low: 4, high: 7, include_low: false, include_high: false };
///
/// // 区间 (4, 7]（不包含下限，包含上限）
/// let config3 = RangeConfig { low: 4, high: 7, include_low: false, include_high: true };
/// ```
#[derive(Copy, Clone)]
pub struct RangeConfig<T> {
    /// 区间下限
    pub low: T,
    /// 区间上限
    pub high: T,
    /// 是否包含下限（`true` = `≥`，`false` = `>`）
    pub include_low: bool,
    /// 是否包含上限（`true` = `≤`，`false` = `<`）
    pub include_high: bool,
}

impl<T> RangeConfig<T> {
    /// 创建包含两端的区间 `[low, high]`
    pub fn inclusive(low: T, high: T) -> Self {
        Self { low, high, include_low: true, include_high: true }
    }

    /// 创建不包含两端的区间 `(low, high)`
    pub fn exclusive(low: T, high: T) -> Self {
        Self { low, high, include_low: false, include_high: false }
    }
}