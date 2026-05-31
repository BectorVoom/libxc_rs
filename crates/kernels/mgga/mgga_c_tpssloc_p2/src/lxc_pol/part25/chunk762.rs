//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 762/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk762<F: Float>(t2427: F, t2655: F, t152: F, t31: F, t185: F, t9288: F, t2448: F, t67: F, t758: F, t2368: F, t2505: F, t745: F) -> (F, F, F, F) {
    let t9896 = F::cast_from(12.0_f64) * t2427 * t2655;
    let t9897 = t31 * t152;
    let t9898 = t185 * t9288;
    let t9900 = F::cast_from(24.0_f64) * t9897 * t9898;
    let t9901 = t2448 * t67;
    let t9902 = t9901 * t758;
    let t9903 = F::cast_from(0.54934341918019635162e-3_f64) * t9902;
    let t9905 = t2368 * t745 * t2505;
    (t9896, t9900, t9903, t9905)
}
