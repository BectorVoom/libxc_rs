//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 574/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk574<F: Float>(t1489: F, t2563: F, t131: F, t2570: F, t205: F, t1484: F, t213: F, t118: F, t794: F, t2576: F, t1493: F, t225: F, t1496: F, t1499: F, t68: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4124 = t2563 * t1489;
    let t4126 = t2570 * t131;
    let t4127 = t205 * t4126;
    let t4128 = t213 * t1484;
    let t4134 = t118 * t794 * t1484;
    let t4135 = t2576 * t4134;
    let t4147 = t1493 * t225;
    let t4152 = t2563 * t1496;
    let t4166 = t1499 * t68;
    (t4124, t4126, t4127, t4128, t4134, t4135, t4147, t4152, t4166)
}
