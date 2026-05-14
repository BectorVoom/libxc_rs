//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1279/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1279<F: Float>(t1983: F, t31035: F, t7940: F, t31304: F, t7754: F, t33366: F, t6876: F, t24994: F, t8606: F, t24996: F, t2075: F, t26135: F, t652: F, t2314: F, t33620: F, t4034: F) -> (F, F, F, F, F, F, F) {
    let t122692 = t1983 * t7940 * t31035;
    let t122696 = t31304 * t7754;
    let t122697 = t6876 * t33366;
    let t122698 = t8606 * t24994;
    let t122700 = 6.0 * t122698 * t24996;
    let t122706 = 2.0 * t652 * t2075 * t26135;
    let t122708 = 2.0 * t2314 * t33620;
    let t122710 = 2.0 * t4034 * t33620;
    (t122692, t122696, t122697, t122700, t122706, t122708, t122710)
}
