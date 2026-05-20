//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1901/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1901<F: Float>(t1985: F, t20009: F, t214: F, t225: F, t567: F, t3886: F, t6439: F, t1307: F, t22633: F, t22635: F, t26193: F, t26202: F) -> (F, F, F) {
    let t97604 = t1985 * t214 * t20009 * t225 * t567;
    let t97608 = t3886 * t6439;
    let t97611 = t22633 * t22635 * t97608 * t1307;
    let t97616 = t1985 * t26193 * t26202;
    (t97604, t97611, t97616)
}
