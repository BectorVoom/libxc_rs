//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 830/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk830<F: Float>(t29535: F, t3598: F, t6267: F, t7301: F, t7300: F, t2123: F, t6140: F, t1716: F, t8010: F, t27382: F, t2130: F, t46: F) -> (F, F, F, F, F, F) {
    let t29536 = t3598 * t29535;
    let t29545 = t7301 * t6267;
    let t29546 = t7300 * t29545;
    let t29551 = t6140 * t2123;
    let t29554 = t1716 * t8010;
    let t29557 = t1716 * t27382;
    let t29560 = t2130 * t46;
    (t29536, t29546, t29551, t29554, t29557, t29560)
}
