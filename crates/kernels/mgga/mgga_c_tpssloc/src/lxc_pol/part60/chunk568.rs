//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 568/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk568<F: Float>(t2108: F, t33: F, t2240: F, t50: F, t55: F, t111: F, t2113: F) -> (F, F, F, F) {
    let t7245 = t33 * t2108;
    let t7246 = t2240 * t7245;
    let t7251 = t50 * t55;
    let t7266 = t2113 * t111;
    (t7245, t7246, t7251, t7266)
}
