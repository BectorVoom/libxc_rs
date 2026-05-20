//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1968/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1968<F: Float>(t16048: F, t5335: F, t3793: F, t1332: F, t5333: F, t5230: F, t68: F) -> (F, F, F, F) {
    let t16049 = t5335 * t16048;
    let t16052 = t5335 * t3793;
    let t16055 = t1332 * t5333;
    let t16060 = t5230 * t68;
    (t16049, t16052, t16055, t16060)
}
