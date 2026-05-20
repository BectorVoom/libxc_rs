//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1326/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1326<F: Float>(t118454: F, t23788: F, t2314: F, t32677: F, t4034: F, t5107: F, t652: F, t8326: F, t1845: F, t6995: F, t1799: F, t1437: F, t31: F) -> (F, F, F, F, F, F, F) {
    let t119780 = t23788 * t118454;
    let t119824 = F::new(2.0) * t2314 * t32677;
    let t119826 = F::new(2.0) * t4034 * t32677;
    let t119830 = F::new(2.0) * t652 * t5107 * t8326;
    let t119832 = t1845 * t6995;
    let t119853 = t1799 * t6995;
    let t119878 = t1437 * t31;
    (t119780, t119824, t119826, t119830, t119832, t119853, t119878)
}
