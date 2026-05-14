//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1092/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1092<F: Float>(t10770: F, t1561: F, t10660: F, t1543: F, t10402: F, t14618: F, t14608: F, t1020: F, t1616: F, t248: F, t43216: F, t10882: F, t48569: F, t10875: F, t1606: F, t2402: F, t973: F) -> (F, F, F, F, F, F, F, F) {
    let t49430 = t1561 * t10770;
    let t49489 = t1543 * t10660;
    let t49929 = t14618 * t10402;
    let t49934 = t14608 * t10402;
    let t50181 = t1020 * t248 * t43216 * t1616;
    let t50193 = t48569 * t10882;
    let t50265 = t48569 * t10875;
    let t50425 = t973 * t2402 * t1606;
    (t49430, t49489, t49929, t49934, t50181, t50193, t50265, t50425)
}
