//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1205/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1205<F: Float>(t2693: F, t5576: F, t2627: F, t5631: F, t10143: F, t5660: F, t2394: F, t5678: F) -> (F, F, F, F) {
    let t59288 = t5576 * t2693;
    let t59355 = t2627 * t5631;
    let t59564 = t5660 * t10143;
    let t59657 = t2394 * t5678;
    (t59288, t59355, t59564, t59657)
}
