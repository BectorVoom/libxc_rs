//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 787/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk787<F: Float>(t3: F, t7758: F, t1873: F, t5371: F, t1458: F, t3941: F, t1401: F, t7467: F, t577: F, t7010: F, t2018: F, t3701: F) -> (F, F, F, F) {
    let t7759 = t3 * t7758;
    let t7768 = F::new(0.135e2) * t5371 * t1873;
    let t7769 = t1873 * t1458;
    let t7771 = F::new(27.0) * t3941 * t7769;
    let t7773 = F::new(0.135e2) * t1401 * t7467;
    let t7774 = F::new(0.45e1) * t7758 * t577 + F::new(0.135e2) * t7010 * t1458 + t7768 + t7771 + t7773;
    let t8643 = t3701 * t2018;
    (t7759, t7769, t7774, t8643)
}
