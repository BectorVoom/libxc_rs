//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1111/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1111<F: Float>(t12254: F, t2863: F, t9493: F, t1519: F, t2911: F, t1543: F, t2975: F, t1053: F, t4117: F, t1523: F, t2954: F, t1063: F, t12000: F, t12219: F, t12222: F, t12243: F, t12246: F, t12250: F, t12253: F, t2950: F, t2955: F, t2958: F, t2999: F, t4120: F, t9380: F) -> (F, F, F) {
    let t12255 = t12254 * t2863;
    let t12257 = F::cast_from(0.51726012919273400301e3_f64) * t9493 * t12255;
    let t12258 = t1519 * t2863;
    let t12260 = F::new(6.0) * t2911 * t12258;
    let t12261 = t1543 * t2975;
    let t12264 = t4117 * t1053;
    let t12269 = t1523 * t2954;
    let t12273 = F::cast_from(0.10254018858216406658e4_f64) * t9380 * t12219 + F::new(6.0) * t2955 * t12222 + t12243 + t12246 - t12250 - t12253 - t12257 - t12260 + F::cast_from(0.35089341735807877242e1_f64) * t2999 * t12261 + F::new(2.0) * t12264 * t1063 + F::new(1.0) * t4120 * t2950 + F::cast_from(0.32163958997385070134e2_f64) * t12269 * t2958 - F::cast_from(0.19751673498613801407e-1_f64) * t12000;
    (t12257, t12260, t12273)
}
