//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1030/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1030<F: Float>(t11100: F, t866: F, t846: F, t11002: F, t11004: F, t10982: F, t10980: F, t10986: F, t11010: F, t11015: F, t11020: F, t11024: F, t11028: F, t11033: F, t11037: F, t8605: F, t8607: F, t8616: F, t8618: F, t8756: F) -> (F, F) {
    let t11101 = t11100 * t866;
    let t11103 = F::cast_from(1.0_f64) * t846 * t11101;
    let t11109 = F::cast_from(0.41203703703703703704e-2_f64) * t11002;
    let t11110 = F::cast_from(0.12361111111111111111e-1_f64) * t11004;
    let t11111 = F::cast_from(0.61805555555555555556e-2_f64) * t10982;
    let t11120 = -t8756 - F::cast_from(0.82407407407407407407e-2_f64) * t8616 + F::cast_from(0.20601851851851851852e-2_f64) * t8607 - F::cast_from(0.61805555555555555556e-2_f64) * t8618 + F::cast_from(0.30902777777777777778e-2_f64) * t8605 - F::cast_from(0.41203703703703703704e-2_f64) * t10980 + t11109 - t11110 + t11111 - F::cast_from(0.10300925925925925926e-1_f64) * t11010 + F::cast_from(0.37083333333333333333e-1_f64) * t11015 - F::cast_from(0.12361111111111111111e-1_f64) * t11020 - F::cast_from(0.61805555555555555555e-2_f64) * t11024 - F::cast_from(0.55625000000000000001e-1_f64) * t11028 + F::cast_from(0.37083333333333333334e-1_f64) * t11033 + F::cast_from(0.18541666666666666667e-1_f64) * t11037 - F::cast_from(0.92708333333333333333e-2_f64) * t10986;
    (t11103, t11120)
}
