//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1170/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1170<F: Float>(t26142: F, t8526: F, t22461: F, t7468: F, t2314: F, t32677: F, t4034: F, t5107: F, t652: F, t8326: F, t32783: F, t6876: F, t1845: F, t6995: F, t26161: F, t26162: F) -> (F, F, F, F, F, F, F) {
    let t119810 = 4.0 * t8526 * t26142;
    let t119811 = t22461 * t7468;
    let t119824 = 2.0 * t2314 * t32677;
    let t119826 = 2.0 * t4034 * t32677;
    let t119830 = 2.0 * t652 * t5107 * t8326;
    let t119831 = t6876 * t32783;
    let t119832 = t1845 * t6995;
    let t119835 = 4.0 * t26161 * t26162 * t119832;
    (t119810, t119811, t119824, t119826, t119830, t119831, t119835)
}
