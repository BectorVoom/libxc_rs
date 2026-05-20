//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 782/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk782<F: Float>(t28378: F, t28405: F, t235: F, t5612: F, t6657: F, t5617: F, t23008: F, t5585: F, t16758: F, t232: F, t6646: F, t1888: F) -> (F, F, F, F, F, F) {
    let t28406 = t28378 + t28405;
    let t28407 = t235 * t28406;
    let t28409 = t6657 * t5612;
    let t28411 = t6657 * t5617;
    let t28413 = t23008 * t5585;
    let t28418 = t16758 * t232;
    let t28419 = t6646 * t28418;
    let t28420 = t1888 * t28419;
    (t28406, t28407, t28409, t28411, t28413, t28420)
}
