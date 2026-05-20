//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 917/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk917<F: Float>(t5480: F, t9398: F, t6320: F, t67: F, t758: F, t12061: F, t6305: F, t12072: F, t6312: F, t750: F, t17: F, t588: F, t6328: F) -> (F, F, F, F, F, F, F, F) {
    let t19513 = t9398 * t5480;
    let t19541 = t6320 * t67;
    let t19542 = t19541 * t758;
    let t19547 = t12061 * t6305;
    let t19559 = t12072 * t6312;
    let t19575 = t6320 * t750;
    let t19576 = t17 * t19575;
    let t19591 = t588 * t6328;
    (t19513, t19541, t19542, t19547, t19559, t19575, t19576, t19591)
}
