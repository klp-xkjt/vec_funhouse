use crate::types::{RangeConfig, RangeMode};
use crate::filter_range;

/// 按顺序应用多个区间过滤规则
///
/// 按照 `rules` 切片中的顺序，依次对 `vector` 应用每个过滤规则。
/// 后面的规则会作用于前面规则过滤后的结果上。
///
/// # 参数
///
/// * `vector` - 要过滤的可变向量引用
/// * `rules` - 规则切片，每个元素是 (区间配置, 过滤模式) 元组
///
/// # 类型约束
///
/// * `T` - 必须实现 `PartialOrd`（可比较大小）
///
/// # 示例
///
/// ```
/// use vec_funhouse::{filter_multi_range, RangeConfig, RangeMode};
///
/// let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
///
/// let config1 = RangeConfig { low: 2, high: 7, include_low: true, include_high: true };
/// let config2 = RangeConfig { low: 4, high: 5, include_low: true, include_high: true };
///
/// let rules = [(config1, RangeMode::KeepInside), (config2, RangeMode::KeepOutside)];
/// filter_multi_range(&mut v, &rules);
///
/// assert_eq!(v, vec![2, 3, 6, 7]);
/// ```
///
/// # 注意
///
/// - 规则的顺序会影响最终结果，因为过滤是按顺序执行的
/// - 每个 `RangeConfig` 会以引用方式传递给 `filter_range`
///
/// # 性能提示
///
/// 对于大量规则的场景，考虑将多个规则合并为一次遍历（需要手动实现组合逻辑），
/// 当前实现会多次遍历向量，但逻辑清晰易懂。
pub fn filter_multi_range<T>(vector: &mut Vec<T>, rules: &[(RangeConfig<T>, RangeMode)])
where
    T: PartialOrd,
{
    for (config, mode) in rules.iter() {
        filter_range(vector, &config, *mode);
        // 注意：config 类型是 &RangeConfig<T>，mode 是 &RangeMode，
        // 但 filter_range 接受 &RangeConfig<T> 和 RangeMode（Copy），
        // 所以 mode 需要解引用（*mode）或传入 mode.clone()
    }
}


/// 按顺序应用多个闭包过滤条件
///
/// 按照 `rules` 切片中的顺序，依次用每个闭包条件过滤 `vector`。
/// 后面的条件会作用于前面条件过滤后的结果上。
///
/// # 参数
///
/// * `vector` - 要过滤的可变向量引用
/// * `rules` - 闭包切片，每个闭包接收 `&T` 并返回 `bool`
///
/// # 类型约束
///
/// * `T` - 必须实现 `PartialOrd`（可比较大小）
/// * `F` - 闭包类型，必须实现 `Fn(&T) -> bool`
///
/// # 示例
///
/// ```
/// use vec_funhouse::filter_multi_closure;
///
/// let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
///
/// let rules: [&dyn Fn(&i32) -> bool; 3] = [
///     &|x| *x > 3,      // 保留大于 3 的数
///     &|x| *x < 8,      // 再保留小于 8 的数
///     &|x| *x != 5,     // 再排除 5
/// ];
///
/// filter_multi_closure(&mut v, &rules);
/// assert_eq!(v, vec![4, 6, 7]);
/// ```
///
/// # 注意
///
/// * 闭包的顺序会影响最终结果
/// * 如果需要更丰富的区间过滤，可以考虑使用 `filter_multi_range`
///
pub fn filter_multi_closure<T, F>(vector: &mut Vec<T>, rules: &[F]) 
where
    T: PartialOrd,
    F: Fn(&T) -> bool,
    {
        for rule in rules {
            vector.retain(rule);
        }
    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_multi_range_order_matters() {
        let mut original = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
        let config_small = RangeConfig { low: 2, high: 7, include_low: true, include_high: true };
        let config_large = RangeConfig { low: 4, high: 5, include_low: true, include_high: true };
    
        let rules1 = [(config_small, RangeMode::KeepInside), (config_large, RangeMode::KeepOutside)];
        filter_multi_range(&mut original, &rules1);
    
        assert_eq!(original, vec![2, 3, 6, 7])
    }

    #[test]
    fn filter_multi_closure_test() {
        let mut vector = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        
        let rules: [&dyn Fn(&i32) -> bool; 3] = [
            &|x| *x > 3,
            &|x| *x < 8,
            &|x| *x != 5,
        ];
        
        filter_multi_closure(&mut vector, &rules);
        println!("结果: {:?}", vector);
        
        assert_eq!(vector, vec![4, 6, 7]);
    }
}