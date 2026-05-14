//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1142/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1142<F: Float>(t119827: F, t119863: F, t120015: F, t120059: F, t120098: F, t120667: F, t120713: F, t120755: F, t1858: F, t8496: F, t2029: F, t7758: F, t112516: F, t112518: F, t114439: F, t114441: F, t118373: F, t1404: F, t26510: F, t3: F, t31254: F, t33165: F, t5381: F, t580: F, t8497: F) -> (F, F) {
    let t120758 = t119827 + t119863 + t120015 + t120059 + t120098 + t120667 + t120713 + t120755;
    let t120762 = t8496 * t1858;
    let t120767 = t7758 * t2029;
    let t120771 = t120758 * t3 * t580 + t1404 * t33165 + t1858 * t31254 + 2.0 * t2029 * t26510 + t5381 * t8497 + t112516 + t112518 + 2.0 * t114439 + 2.0 * t114441 + t118373 + t120762 + 2.0 * t120767;
    (t120758, t120771)
}
