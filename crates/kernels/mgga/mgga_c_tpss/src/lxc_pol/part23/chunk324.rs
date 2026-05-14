//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 324/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk324<F: Float>(t1015: F, t1038: F, t141: F, t1017: F, t1028: F, t1030: F, t1033: F, t1037: F) -> (F, F, F) {
    let t1039 = t1038 * t1015;
    let t1040 = t141 * t1039;
    let t1042 = 0.1898925e1 * t1028 - t1030 + 0.29896666666666666667e0 * t1017 + 0.3071625e0 * t1033 - t1037 + 0.82156666666666666667e-1 * t1040;
    (t1039, t1040, t1042)
}
