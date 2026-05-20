//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1394/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1394<F: Float>(t7015: F, t96334: F, t7769: F, t85416: F, t24972: F, t26550: F, t116343: F, t120809: F, t120811: F, t120812: F, t120815: F, t120818: F, t120820: F, t120823: F, t5376: F) -> F {
    let t123294 = t96334 * t7015;
    let t123296 = t85416 * t7769;
    let t123298 = t24972 * t26550;
    let t123304 = t120809 + t120811 + F::new(27.0) * t123294 + F::new(27.0) * t123296 + F::new(27.0) * t123298 + F::new(27.0) * t116343 * t5376 + F::new(0.135e2) * t120812 + F::new(0.135e2) * t120815 + t120818 + t120820 + t120823;
    t123304
}
