//! Property-based tests for the 1's complement word model.

use agc_math::ones_complement::*;
use proptest::prelude::*;

/// Strategy to generate valid AgcWord values (raw 15-bit patterns).
fn agc_word_strategy() -> impl Strategy<Value = AgcWord> {
    (0u16..=AgcWord::MASK).prop_map(AgcWord::from_raw)
}

/// Strategy for non-zero AgcWords.
fn nonzero_agc_word() -> impl Strategy<Value = AgcWord> {
    (1u16..=0x7FFEu16).prop_map(AgcWord::from_raw)
}

proptest! {
    /// Negation is an involution: negate(negate(x)) = x.
    #[test]
    fn negate_involution(w in agc_word_strategy()) {
        prop_assert_eq!(w.negate().negate(), w);
    }

    /// Negation flips the sign bit (except for +0 and -0 which swap).
    #[test]
    fn negate_flips_sign(w in nonzero_agc_word()) {
        prop_assert_ne!(w.is_positive(), w.negate().is_positive());
    }

    /// Magnitude is always non-negative and preserved through negation.
    #[test]
    fn magnitude_preserved_by_negate(w in agc_word_strategy()) {
        prop_assert_eq!(w.magnitude(), w.negate().magnitude());
    }

    /// to_i16 roundtrips through from_i16 for non-negative-zero values.
    #[test]
    fn to_i16_from_i16_roundtrip(val in -16383i16..=16383) {
        let word = AgcWord::from_i16(val);
        prop_assert_eq!(word.to_i16(), val);
    }

    /// CCS always returns branch 0-3.
    #[test]
    fn ccs_branch_in_range(w in agc_word_strategy()) {
        let (branch, _) = w.ccs();
        prop_assert!(branch <= 3);
    }

    /// CCS diminished absolute value is always non-negative.
    #[test]
    fn ccs_dabs_non_negative(w in agc_word_strategy()) {
        let (_, dabs) = w.ccs();
        prop_assert!(dabs.is_positive() || dabs.is_pos_zero());
    }

    /// Absolute value is always positive or +0.
    #[test]
    fn abs_is_positive(w in agc_word_strategy()) {
        let a = w.abs();
        prop_assert!(a.is_positive() || a.is_pos_zero());
    }

    /// DAS of x and 0 returns x (additive identity).
    #[test]
    fn das_additive_identity(
        maj in -16383i16..=16383,
        min in -16383i16..=16383,
    ) {
        let a_maj = AgcWord::from_i16(maj);
        let a_min = AgcWord::from_i16(min);
        let (r_maj, r_min, _) = das(a_maj, a_min, AgcWord::POS_ZERO, AgcWord::POS_ZERO);
        prop_assert_eq!(r_maj, a_maj);
        prop_assert_eq!(r_min, a_min);
    }
}

#[test]
fn test_neg_zero_ccs_branch3() {
    let (branch, dabs) = AgcWord::NEG_ZERO.ccs();
    assert_eq!(branch, 3);
    assert!(dabs.is_pos_zero());
}

#[test]
fn test_pos_zero_ccs_branch1() {
    let (branch, dabs) = AgcWord::POS_ZERO.ccs();
    assert_eq!(branch, 1);
    assert!(dabs.is_pos_zero());
}
