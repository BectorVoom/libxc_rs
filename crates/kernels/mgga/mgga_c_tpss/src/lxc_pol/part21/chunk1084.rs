//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1084/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1084<F: Float>(t12241: F, t408: F, t1505: F, t2861: F, t2864: F, t2913: F, t4104: F, t1042: F, t2911: F, t2905: F, t4108: F, t1518: F, t9495: F, t2863: F, t9493: F, t1519: F) -> (F, F, F, F, F, F) {
    let t12243 = 0.621814e-1 * t12241 * t408;
    let t12244 = t1505 * t2861;
    let t12246 = 2.0 * t12244 * t2864;
    let t12247 = t4104 * t2913;
    let t12248 = t12247 * t1042;
    let t12250 = 0.32163958997385070134e2 * t2911 * t12248;
    let t12251 = t4108 * t2905;
    let t12253 = 0.16081979498692535067e2 * t2911 * t12251;
    let t12254 = t1518 * t9495;
    let t12255 = t12254 * t2863;
    let t12257 = 0.51726012919273400301e3 * t9493 * t12255;
    let t12258 = t1519 * t2863;
    (t12243, t12246, t12250, t12253, t12257, t12258)
}
