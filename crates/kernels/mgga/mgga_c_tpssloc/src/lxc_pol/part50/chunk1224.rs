//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1224/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1224<F: Float>(t5107: F, t652: F, t8326: F, t32783: F, t6876: F, t1845: F, t6995: F, t26161: F, t26162: F, t31537: F, t7468: F, t31540: F) -> (F, F, F, F, F) {
    let t119830 = F::new(2.0) * t652 * t5107 * t8326;
    let t119831 = t6876 * t32783;
    let t119832 = t1845 * t6995;
    let t119835 = F::new(4.0) * t26161 * t26162 * t119832;
    let t119837 = F::new(4.0) * t31537 * t7468;
    let t119839 = F::new(4.0) * t31540 * t7468;
    (t119830, t119831, t119835, t119837, t119839)
}
