//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 930/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk930<F: Float>(t30: F, t502: F, t33: F, t504: F, t1173: F, t3197: F, t1193: F, t8021: F, t2215: F, t3178: F, t2345: F, t3204: F, t540: F, t1183: F, t2331: F, t489: F) -> (F, F, F, F, F, F, F, F) {
    let t9856 = 1.0 / t502 / t30;
    let t9868 = 1.0 / t504 / t33;
    let t9883 = t1173 * t3197;
    let t9886 = 0.10389515463408878255e3 * t1193 * t8021;
    let t9887 = t3178 * t2215;
    let t9890 = t3178 * t2345;
    let t9895 = 1.0 / t3204 / t540;
    let t9899 = t1183 * t2331;
    let t9900 = t489 * t9899;
    (t9856, t9868, t9883, t9886, t9887, t9890, t9895, t9900)
}
