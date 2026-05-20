//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2642/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2642<F: Float>(t16391: F, t16398: F, t12283: F, t16244: F, t3862: F, t5231: F, t16356: F, t3726: F, t12328: F, t1815: F, t16397: F, t3777: F, t5252: F) -> (F, F, F, F, F, F) {
    let t54750 = t16398 * t16391;
    let t54764 = t12283 * t16244;
    let t54785 = t5231 * t3862;
    let t54787 = t3726 * t16356;
    let t54793 = t1815 * t12328;
    let t54801 = t3777 * t16397 * t5252;
    (t54750, t54764, t54785, t54787, t54793, t54801)
}
