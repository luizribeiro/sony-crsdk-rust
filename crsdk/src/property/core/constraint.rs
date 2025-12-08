//! Value constraints for camera properties.

/// Constraint on what values a property can have.
///
/// The SDK provides value constraints in two forms:
/// - **Discrete**: A list of specific allowed values
/// - **Range**: A min/max/step triplet defining a numeric range
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ValueConstraint {
    /// No constraint information available
    #[default]
    None,
    /// Discrete list of allowed values
    Discrete(Vec<u64>),
    /// Numeric range with min, max, and step
    Range {
        /// Minimum allowed value
        min: i64,
        /// Maximum allowed value
        max: i64,
        /// Step/increment between valid values
        step: i64,
    },
}

impl ValueConstraint {
    /// Check if a value satisfies this constraint
    pub fn is_valid(&self, value: u64) -> bool {
        match self {
            Self::None => true,
            Self::Discrete(values) => values.is_empty() || values.contains(&value),
            Self::Range { min, max, step } => {
                let v = value as i64;
                if v < *min || v > *max {
                    return false;
                }
                if *step == 0 {
                    return true;
                }
                (v - min) % step == 0
            }
        }
    }

    /// Check if this constraint is empty (no values or no range)
    pub fn is_empty(&self) -> bool {
        match self {
            Self::None => true,
            Self::Discrete(values) => values.is_empty(),
            Self::Range { .. } => false,
        }
    }

    /// Get all valid values if this is a discrete constraint
    pub fn discrete_values(&self) -> Option<&[u64]> {
        match self {
            Self::Discrete(values) => Some(values),
            _ => None,
        }
    }

    /// Get range parameters if this is a range constraint
    pub fn range_params(&self) -> Option<(i64, i64, i64)> {
        match self {
            Self::Range { min, max, step } => Some((*min, *max, *step)),
            _ => None,
        }
    }

    /// Expand a range constraint into discrete values (for UI display)
    ///
    /// Returns `None` if this is not a range constraint or if the range
    /// would produce too many values (> 1000).
    pub fn expand_range(&self) -> Option<Vec<i64>> {
        match self {
            Self::Range { min, max, step } => {
                if *step == 0 {
                    return None;
                }
                let count = ((max - min) / step) + 1;
                if count > 1000 {
                    return None;
                }
                let mut values = Vec::with_capacity(count as usize);
                let mut v = *min;
                while v <= *max {
                    values.push(v);
                    v += step;
                }
                Some(values)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_constraint_range_with_step() {
        let constraint = ValueConstraint::Range {
            min: 0,
            max: 10,
            step: 2,
        };
        assert!(constraint.is_valid(0));
        assert!(constraint.is_valid(2));
        assert!(constraint.is_valid(4));
        assert!(constraint.is_valid(10));
        assert!(!constraint.is_valid(1));
        assert!(!constraint.is_valid(3));
        assert!(!constraint.is_valid(11));

        let expanded = constraint.expand_range().unwrap();
        assert_eq!(expanded, vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_value_constraint_range_signed() {
        let constraint = ValueConstraint::Range {
            min: -10,
            max: 10,
            step: 5,
        };
        assert!(constraint.is_valid((-10_i64) as u64));
        assert!(constraint.is_valid((-5_i64) as u64));
        assert!(constraint.is_valid(0));
        assert!(constraint.is_valid(5));
        assert!(constraint.is_valid(10));

        let expanded = constraint.expand_range().unwrap();
        assert_eq!(expanded, vec![-10, -5, 0, 5, 10]);
    }
}
