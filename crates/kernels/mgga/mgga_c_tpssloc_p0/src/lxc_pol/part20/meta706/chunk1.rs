//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2691/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2691<F: Float>(t3862: F, t5231: F, t16356: F, t3726: F, t12328: F, t1815: F, t16397: F, t3777: F, t5252: F, t1336: F, t2691: F, t3788: F) -> (F, F, F, F, F) {
    let t54785 = t5231 * t3862;
    let t54786 = F::new(119.0) / F::new(4608.0) * t54785;
    let t54787 = t3726 * t16356;
    let t54793 = t1815 * t12328;
    let t54801 = t3777 * t16397 * t5252;
    let t54811 = t1336 * t3788 * t2691 * t5252;
    (t54786, t54787, t54793, t54801, t54811)
}
