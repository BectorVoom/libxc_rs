//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1320/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1320<F: Float>(t232: F, t76001: F, t2632: F, t76085: F, t13283: F, t1510: F, t20963: F, t20969: F, t20981: F, t2630: F, t2643: F, t2645: F, t41096: F, t4167: F, t4178: F, t5527: F, t5544: F, t58809: F, t67607: F, t67644: F, t67976: F, t67978: F, t67980: F, t76090: F, t817: F, t819: F, t820: F, t843: F, t9607: F, t9974: F) -> (F, F, F) {
    let t76274 = t76001 * t232;
    let t76290 = t76085 * t2632;
    let t76295 = -t4178 * t2645 * t67607 * t20981 / F::new(32.0) + t2643 * t2645 * t67644 * t1510 / F::new(192.0) + F::new(7.0) / F::new(384.0) * t67976 - F::new(7.0) / F::new(192.0) * t67978 - F::new(7.0) / F::new(192.0) * t67980 + t41096 + F::new(119.0) / F::new(1152.0) * t58809 - t4167 * t20969 / F::new(768.0) - t817 * t819 * t820 * t76274 / F::new(1024.0) + t13283 * t20963 / F::new(128.0) - F::new(15.0) / F::new(64.0) * t843 * t9607 * t820 * t5527 * t5544 - F::new(3.0) / F::new(256.0) * t9974 * t819 * t820 * t76090 + F::new(7.0) / F::new(1536.0) * t2630 * t819 * t820 * t76290;
    (t76274, t76290, t76295)
}
