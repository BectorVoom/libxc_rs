//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1592/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1592<F: Float>(t1013: F, t363: F, t3034: F, t6793: F) -> (F, F, F, F) {
    let t10473 = t1013 * t1013;
    let t10474 = F::new(1.0) / t10473;
    let t10475 = t10474 * t363;
    let t10477 = F::new(1.0) / t3034 / t6793;
    (t10473, t10474, t10475, t10477)
}
