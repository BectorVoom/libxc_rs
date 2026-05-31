//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 167/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk167<F: Float>(t592: F, t14: F, t2: F, t21: F, t15: F, t583: F) -> (F, F, F, F) {
    let t593 = F::cast_from(2.0_f64) * t592;
    let t594 = t14 * t2;
    let t596 = F::cast_from(0.1356e2_f64) * t594 * t21;
    let t597 = t15 * t583;
    let t598 = F::cast_from(1.0_f64) / t597;
    (t593, t596, t597, t598)
}
