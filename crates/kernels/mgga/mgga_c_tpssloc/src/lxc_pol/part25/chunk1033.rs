//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1033/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1033<F: Float>(t10046: F, t1880: F, t1894: F, t214: F, t1879: F, t80845: F, t1906: F, t23035: F, t23153: F, t2379: F, t6637: F, t23229: F, t81715: F, t225: F, t23228: F, t23272: F, t81651: F) -> (F, F, F, F, F, F) {
    let t82043 = t1880 * t214 * t1894 * t10046;
    let t82045 = t80845 * t1879;
    let t82046 = t82045 * t1906;
    let t82050 = t23035 * t6637 * t23153 * t2379;
    let t82069 = t81715 * t23229;
    let t82074 = t23228 * t225;
    let t82076 = t81651 * t82074 * t23272;
    (t82043, t82045, t82046, t82050, t82069, t82076)
}
