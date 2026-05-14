//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 828/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk828<F: Float>(t2274: F, t50: F, t7245: F, t9239: F, t2109: F, t22550: F, t9231: F, t33: F, t7254: F, t2240: F, t1235: F, t7299: F, t2127: F, t23383: F) -> (F, F, F, F, F, F, F) {
    let t24498 = t50 * t2274;
    let t24514 = t9239 * t7245;
    let t24517 = t2109 * t22550;
    let t24520 = t9231 * t7245;
    let t24525 = t33 * t7254;
    let t24526 = t2240 * t24525;
    let t24567 = t7299 * t1235;
    let t24574 = t2127 * t23383;
    (t24498, t24514, t24517, t24520, t24526, t24567, t24574)
}
