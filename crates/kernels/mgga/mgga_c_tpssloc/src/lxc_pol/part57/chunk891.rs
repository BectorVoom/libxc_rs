//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 891/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk891<F: Float>(t120955: F, t1983: F, t7687: F, t33335: F, t5161: F, t33366: F, t7685: F, t5450: F, t8595: F, t2075: F, t28017: F, t652: F, t5493: F, t33620: F, t4028: F, t22574: F, t33357: F, t33899: F) -> (F, F, F, F, F, F, F, F) {
    let t128438 = 6.0 * t1983 * t120955 * t7687;
    let t128441 = 2.0 * t1983 * t33335 * t5161;
    let t128443 = 2.0 * t7685 * t33366;
    let t128444 = t5450 * t8595;
    let t128449 = 2.0 * t652 * t2075 * t28017;
    let t128452 = 2.0 * t652 * t8595 * t5493;
    let t128454 = 4.0 * t4028 * t33620;
    let t128457 = 6.0 * t22574 * t33899 * t33357;
    (t128438, t128441, t128443, t128444, t128449, t128452, t128454, t128457)
}
