//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 955/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk955<F: Float>(t11875: F, t4098: F, t673: F, t4095: F, t1502: F, t2193: F) -> (F, F, F, F, F) {
    let t11876 = 0.39862222222222222222e0 * t11875;
    let t11910 = t673 * t4098;
    let t11911 = 0.21908444444444444444e0 * t11910;
    let t11932 = t673 * t4095;
    let t11938 = t2193 * t1502;
    (t11876, t11910, t11911, t11932, t11938)
}
