//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2172/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2172<F: Float>(t3862: F, t5231: F, t12328: F, t1815: F, t1336: F, t2691: F, t3788: F, t5252: F, t3787: F, t5318: F, t40041: F, t544: F, t68: F) -> (F, F, F, F, F) {
    let t54785 = t5231 * t3862;
    let t54786 = F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t54785;
    let t54793 = t1815 * t12328;
    let t54811 = t1336 * t3788 * t2691 * t5252;
    let t54812 = F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t54811;
    let t54905 = t3787 * t5318;
    let t54963 = t544 * t68 * t40041;
    (t54786, t54793, t54812, t54905, t54963)
}
