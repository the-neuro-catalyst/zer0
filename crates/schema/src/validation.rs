// crates/schema/src/axioms.rs

use log::info;

use rust_decimal::prelude::*;

use rust_decimal_macros::dec;

/// Validation error for Axiomatic calculations.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub fn calculate_process_efficiency_factor(
    entropy: Decimal,
    latency: Decimal,
    _sigma_comp: Decimal,
) -> Result<Decimal, ValidationError> {
    if entropy.is_sign_negative() {
        return Err(ValidationError::InvalidInput("Metrics cannot be negative".into()));
    }
    // Stub Logic: Simple ratio calculation
    let safe_latency = if latency.is_zero() { dec!(1.0) } else { latency };
    let result = entropy / safe_latency;

    info!("VALIDATION [Efficiency]: Load={}, Latency={} => Score={}", entropy, latency, result);

    Ok(result)
}

pub fn calculate_focus_ratio(
    focus_vectors: &[Decimal],
    _delta_omega: Decimal,
) -> Result<Decimal, ValidationError> {
    // Stub Logic: Average calculation
    let sum: Decimal = focus_vectors.iter().sum();
    let count = Decimal::from(focus_vectors.len());

    let result = if count.is_zero() { dec!(0.0) } else { sum / count };

    Ok(result)
}

pub fn calculate_state_adjustment(
    resonance_states: &[Decimal],
    _energy_released: Decimal,
    _zeta_res: Decimal,
    _psi_resonance: Decimal,
) -> Result<Decimal, ValidationError> {
    // Stub Logic: Summation
    let result: Decimal = resonance_states.iter().sum();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rust_decimal_macros::dec;

    #[test]
    fn test_efficiency_calculation() {
        let entropy = dec!(10.0);
        let latency = dec!(2.0);
        let result = calculate_process_efficiency_factor(entropy, latency, dec!(1.0)).unwrap();
        assert_eq!(result, dec!(5.0));
    }

    #[test]
    fn test_negative_entropy_fails() {
        let entropy = dec!(-1.0);
        let latency = dec!(1.0);
        let result = calculate_process_efficiency_factor(entropy, latency, dec!(1.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_focus_ratio_average() {
        let vectors = vec![dec!(1.0), dec!(2.0), dec!(3.0)];
        let result = calculate_focus_ratio(&vectors, dec!(1.0)).unwrap();
        assert_eq!(result, dec!(2.0));
    }
}
