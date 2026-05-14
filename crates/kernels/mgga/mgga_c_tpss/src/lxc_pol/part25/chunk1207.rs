//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1207/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1207<F: Float>(t19817: F, t69881: F, t4701: F, t580: F, t14029: F, t30: F, t21298: F, t5570: F, t14322: F, t17964: F, t14326: F, t14343: F, t19703: F, t14189: F, t14181: F, t4724: F, t61033: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t69882 = t19817 * t69881;
    let t69887 = t580 * t4701;
    let t69891 = t30 * t14029;
    let t69912 = t21298 * t5570;
    let t69926 = t17964 * t14322;
    let t69928 = t17964 * t14326;
    let t69930 = t19703 * t14343;
    let t69932 = t17964 * t14189;
    let t69934 = t19703 * t14181;
    let t69936 = t61033 * t4724;
    (t69882, t69887, t69891, t69912, t69926, t69928, t69930, t69932, t69934, t69936)
}
