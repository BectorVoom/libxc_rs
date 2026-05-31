//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1379/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1379<F: Float>(t1983: F, t2019: F, t74014: F, t1390: F, t2018: F, t20356: F, t1845: F, t6330: F, t24995: F, t8643: F, t28239: F, t7685: F) -> (F, F, F, F) {
    let t106964 = t1983 * t2019 * t74014;
    let t106968 = F::cast_from(6.0_f64) * t1983 * t20356 * t2018 * t1390;
    let t106971 = t6330 * t1845;
    let t106974 = F::cast_from(18.0_f64) * t24995 * t8643 * t106971;
    let t106978 = F::cast_from(3.0_f64) * t7685 * t28239;
    (t106964, t106968, t106974, t106978)
}
