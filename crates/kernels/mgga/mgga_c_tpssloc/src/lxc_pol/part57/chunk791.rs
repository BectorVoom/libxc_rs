//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 791/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk791<F: Float>(t343: F, t5836: F, t6734: F, t5842: F, t1941: F, t5904: F, t1011: F, t5872: F, t3131: F, t23512: F, t360: F, t23519: F) -> (F, F, F, F, F) {
    let t28557 = t5836 * t343;
    let t28558 = t28557 * t6734;
    let t28565 = t5842 * t343;
    let t28566 = t28565 * t6734;
    let t28572 = t5904 * t1941;
    let t28576 = t5872 * t1011;
    let t28577 = t28576 * t3131;
    let t28578 = t23512 * t28577;
    let t28581 = t28576 * t360;
    let t28582 = t23519 * t28581;
    (t28558, t28566, t28572, t28578, t28582)
}
