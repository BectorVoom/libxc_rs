//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 459/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk459<F: Float>(t1173: F, t14045: F, t3119: F, t14011: F, t14034: F, t3113: F, t4443: F, t1996: F, t13862: F, t2003: F, t3120: F, t323: F, t1008: F, t140: F, t212: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14046 = t14045 * t1173;
    let t14047 = t14046 * t3119;
    let t14048 = t14011 * t14034;
    let t14049 = t14047 * t14048;
    let t14051 = t3113 * t4443;
    let t14052 = t14051 * t3119;
    let t14053 = t14011 * t1996;
    let t14054 = t14052 * t14053;
    let t14056 = t13862 * t2003;
    let t14057 = t3120 * t14056;
    let t14059 = t14011 * t323;
    let t14060 = t3120 * t14059;
    let t14063 = t212 * t1008 * t140;
    (t14046, t14047, t14048, t14049, t14051, t14052, t14053, t14054, t14056, t14057, t14059, t14060, t14063)
}
