//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 876/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk876<F: Float>(t213: F, t6330: F, t3726: F, t6358: F, t1814: F, t5343: F, t6378: F, t68: F) -> (F, F, F, F) {
    let t19781 = t213 * t6330;
    let t19791 = t3726 * t6358;
    let t19810 = t1814 * t5343;
    let t19815 = t6378 * t68;
    (t19781, t19791, t19810, t19815)
}
