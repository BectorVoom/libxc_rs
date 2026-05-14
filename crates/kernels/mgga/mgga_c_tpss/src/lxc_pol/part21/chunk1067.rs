//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1067/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1067<F: Float>(t4098: F, t673: F, t11888: F, t2895: F, t141: F, t11894: F, t11883: F, t1038: F, t11902: F, t11906: F, t4095: F, t1502: F, t2193: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11910 = t673 * t4098;
    let t11911 = 0.21908444444444444444e0 * t11910;
    let t11912 = t2895 * t11888;
    let t11913 = t141 * t11912;
    let t11915 = t2895 * t11894;
    let t11916 = t141 * t11915;
    let t11918 = t2895 * t11883;
    let t11919 = t141 * t11918;
    let t11921 = t1038 * t11902;
    let t11922 = t141 * t11921;
    let t11924 = t1038 * t11906;
    let t11925 = t141 * t11924;
    let t11932 = t673 * t4095;
    let t11938 = t2193 * t1502;
    (t11910, t11911, t11913, t11916, t11919, t11922, t11925, t11932, t11938)
}
