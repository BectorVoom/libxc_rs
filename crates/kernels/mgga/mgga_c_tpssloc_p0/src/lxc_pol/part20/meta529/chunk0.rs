//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2063/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2063<F: Float>(t12379: F, t3799: F, t12384: F, t3777: F, t3795: F, t12282: F, t3809: F, t12328: F, t1333: F, t1336: F, t2690: F, t3788: F) -> (F, F, F, F, F, F) {
    let t40128 = t3799 * t12379;
    let t40130 = t3777 * t12384;
    let t40131 = t40130 * t3795;
    let t40138 = t3777 * t12282;
    let t40139 = t40138 * t3809;
    let t40145 = t1333 * t12328;
    let t40159 = t1336 * t3788 * t2690;
    (t40128, t40131, t40138, t40139, t40145, t40159)
}
