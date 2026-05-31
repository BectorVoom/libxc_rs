//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1031/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1031<F: Float>(t11120: F, t318: F, t294: F, t2814: F, t4019: F, t11004: F, t10982: F, t10980: F, t10986: F, t11002: F, t11010: F, t11015: F, t11020: F, t11024: F, t11028: F, t11033: F, t11037: F, t8605: F, t8607: F, t8616: F, t8618: F, t8723: F) -> (F, F, F, F) {
    let t11121 = t11120 * t318;
    let t11123 = F::cast_from(0.19751673498613801407e-1_f64) * t294 * t11121;
    let t11124 = t4019 * t2814;
    let t11134 = F::cast_from(0.23744444444444444444e-1_f64) * t11004;
    let t11135 = F::cast_from(0.11872222222222222222e-1_f64) * t10982;
    let t11144 = -t8723 - F::cast_from(0.15829629629629629629e-1_f64) * t8616 + F::cast_from(0.39574074074074074073e-2_f64) * t8607 - F::cast_from(0.11872222222222222222e-1_f64) * t8618 + F::cast_from(0.5936111111111111111e-2_f64) * t8605 - F::cast_from(0.79148148148148148146e-2_f64) * t10980 + F::cast_from(0.79148148148148148146e-2_f64) * t11002 - t11134 + t11135 - F::cast_from(0.19787037037037037037e-1_f64) * t11010 + F::cast_from(0.71233333333333333332e-1_f64) * t11015 - F::cast_from(0.23744444444444444444e-1_f64) * t11020 - F::cast_from(0.11872222222222222222e-1_f64) * t11024 - F::cast_from(0.10685e0_f64) * t11028 + F::cast_from(0.71233333333333333332e-1_f64) * t11033 + F::cast_from(0.35616666666666666666e-1_f64) * t11037 - F::cast_from(0.17808333333333333333e-1_f64) * t10986;
    (t11121, t11123, t11124, t11144)
}
