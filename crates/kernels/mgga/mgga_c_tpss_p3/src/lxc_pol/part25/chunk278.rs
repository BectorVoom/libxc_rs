//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 278/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk278<F: Float>(t833: F, t859: F, t839: F, t850: F, t855: F, t863: F) -> (F, F, F) {
    let t898 = F::cast_from(0.301925e0_f64) * t833;
    let t901 = F::cast_from(0.82785e-1_f64) * t859;
    let t903 = F::cast_from(0.258925e1_f64) * t850 - t898 - F::cast_from(0.301925e0_f64) * t839 + F::cast_from(0.16504875e0_f64) * t855 - t901 - F::cast_from(0.82785e-1_f64) * t863;
    (t898, t901, t903)
}
