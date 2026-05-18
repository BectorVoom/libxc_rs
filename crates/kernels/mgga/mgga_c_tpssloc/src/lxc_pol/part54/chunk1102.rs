//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1102/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1102<F: Float>(t27496: F, t27497: F, t5083: F, t7376: F, t7375: F, t1419: F, t6794: F, t131: F, t467: F, t5075: F, t225: F, t8034: F) -> (F, F, F, F, F) {
    let t27498 = t27496 * t27497;
    let t27501 = t5083 * t7376;
    let t27502 = t7375 * t27501;
    let t27505 = t1419 * t6794;
    let t27506 = t27505 * t131;
    let t27507 = t27506 * t467;
    let t27510 = t5075 * t7376;
    let t27511 = t7375 * t27510;
    let t27516 = t8034 * t225;
    (t27498, t27502, t27507, t27511, t27516)
}
