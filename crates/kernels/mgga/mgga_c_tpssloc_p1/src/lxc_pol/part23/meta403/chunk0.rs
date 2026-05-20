//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1213/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1213<F: Float>(t13965: F, t4641: F, t1020: F, t10508: F, t248: F, t5867: F, t3039: F, t5878: F, t14202: F, t4644: F, t3082: F, t5905: F) -> (F, F, F, F, F) {
    let t62148 = t4641 * t13965;
    let t62177 = t1020 * t248 * t10508 * t5867;
    let t62183 = t3039 * t248 * t10508 * t5878;
    let t62284 = t4644 * t14202;
    let t62360 = t5905 * t3082;
    (t62148, t62177, t62183, t62284, t62360)
}
