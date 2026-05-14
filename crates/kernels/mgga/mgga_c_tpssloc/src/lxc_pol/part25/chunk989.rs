//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 989/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk989<F: Float>(t12178: F, t1339: F, t6936: F, t12168: F, t12303: F, t221: F, t26284: F, t1361: F, t26288: F, t12255: F, t3788: F, t22865: F, t6604: F, t6937: F, t22776: F, t22779: F) -> (F, F, F, F, F, F, F) {
    let t80925 = t6936 * t1339 * t12178;
    let t80928 = t6936 * t1339 * t12168;
    let t80931 = t26284 * t221 * t12303;
    let t80934 = t26288 * t1361 * t12303;
    let t80937 = t6936 * t3788 * t12255;
    let t80939 = t22865 * t6604;
    let t80940 = t80939 * t6937;
    let t80943 = t22779 * t22776;
    (t80925, t80928, t80931, t80934, t80937, t80940, t80943)
}
