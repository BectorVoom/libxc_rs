//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1151/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1151<F: Float>(t28017: F, t510: F, t652: F, t7685: F, t7756: F, t5493: F, t89: F, t1874: F, t7458: F, t7461: F, t4028: F, t7468: F) -> (F, F, F, F, F, F, F) {
    let t28025 = t510 * t28017;
    let t28027 = F::cast_from(2.0_f64) * t652 * t28025;
    let t28029 = F::cast_from(2.0_f64) * t7685 * t7756;
    let t28030 = t89 * t5493;
    let t28032 = F::cast_from(2.0_f64) * t28030 * t1874;
    let t28034 = F::cast_from(4.0_f64) * t7458 * t7461;
    let t28036 = F::cast_from(4.0_f64) * t4028 * t7468;
    (t28025, t28027, t28029, t28030, t28032, t28034, t28036)
}
