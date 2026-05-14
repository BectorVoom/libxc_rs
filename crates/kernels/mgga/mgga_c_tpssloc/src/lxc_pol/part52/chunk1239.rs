//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1239/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1239<F: Float>(t7769: F, t85416: F, t24972: F, t26550: F, t116343: F, t120809: F, t120811: F, t120812: F, t120815: F, t120818: F, t120820: F, t120823: F, t123294: F, t5376: F, t1873: F, t96311: F) -> (F, F) {
    let t123296 = t85416 * t7769;
    let t123298 = t24972 * t26550;
    let t123304 = t120809 + t120811 + 27.0 * t123294 + 27.0 * t123296 + 27.0 * t123298 + 27.0 * t116343 * t5376 + 0.135e2 * t120812 + 0.135e2 * t120815 + t120818 + t120820 + t120823;
    let t123306 = t96311 * t1873;
    (t123304, t123306)
}
