//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1215/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1215<F: Float>(t3131: F, t5866: F, t3199: F, t61734: F, t3185: F, t2394: F, t5972: F) -> (F, F, F, F) {
    let t62840 = t5866 * t3131;
    let t63004 = t61734 * t3199;
    let t63183 = t61734 * t3185;
    let t63332 = t2394 * t5972;
    (t62840, t63004, t63183, t63332)
}
