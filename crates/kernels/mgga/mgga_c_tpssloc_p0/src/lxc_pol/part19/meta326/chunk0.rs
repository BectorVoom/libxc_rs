//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1157/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1157<F: Float>(t12290: F, t3777: F, t12247: F, t551: F, t236: F, t1336: F, t240: F, t3791: F, t3792: F, t12283: F, t12422: F, t12339: F, t3876: F) -> (F, F, F, F, F, F, F) {
    let t40035 = t3777 * t12290;
    let t40041 = F::cast_from(1.0_f64) / t12247 / t551;
    let t40042 = t40041 * t236;
    let t40044 = t1336 * t40042 * t240;
    let t40045 = t3791 * t3791;
    let t40046 = t3792 * t3792;
    let t40047 = t40045 * t40046;
    let t40052 = t12283 * t12422;
    let t40054 = t12339 * t3876;
    (t40035, t40041, t40044, t40045, t40047, t40052, t40054)
}
