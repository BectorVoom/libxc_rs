//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1308/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1308<F: Float>(t3941: F, t7769: F, t1401: F, t7467: F, t1458: F, t577: F, t7010: F, t7758: F, t7768: F, t2018: F, t3701: F) -> (F, F) {
    let t7771 = F::cast_from(27.0_f64) * t3941 * t7769;
    let t7773 = F::cast_from(0.135e2_f64) * t1401 * t7467;
    let t7774 = F::cast_from(0.45e1_f64) * t7758 * t577 + F::cast_from(0.135e2_f64) * t7010 * t1458 + t7768 + t7771 + t7773;
    let t8643 = t3701 * t2018;
    (t7774, t8643)
}
