//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2794/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2794<F: Float>(t59048: F, t59011: F, t59014: F, t59015: F, t59016: F, t59018: F, t59019: F, t59020: F, t59023: F, t59025: F, t59027: F, t59029: F, t59031: F, t59033: F, t59034: F, t59035: F, t59038: F, t59040: F, t59043: F, t59046: F) -> (F, F) {
    let t59049 = F::cast_from(0.36622894612013090108e-3_f64) * t59048;
    let t59050 = t59011 + t59014 + t59015 + t59016 + t59018 + t59019 + t59020 + t59023 + t59025 + t59027 - t59029 + t59031 + t59033 + t59034 + t59035 + t59038 + t59040 + t59043 - t59046 - t59049;
    (t59049, t59050)
}
