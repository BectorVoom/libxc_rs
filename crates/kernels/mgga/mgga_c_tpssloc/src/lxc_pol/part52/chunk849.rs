//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 849/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk849<F: Float>(t3941: F, t7769: F, t1401: F, t7467: F, t1458: F, t577: F, t7010: F, t7758: F, t7768: F, t1409: F, t1419: F, t56: F, t6503: F, t7251: F) -> (F, F, F, F) {
    let t7771 = F::new(27.0) * t3941 * t7769;
    let t7773 = F::new(0.135e2) * t1401 * t7467;
    let t7774 = F::new(0.45e1) * t7758 * t577 + F::new(0.135e2) * t7010 * t1458 + t7768 + t7771 + t7773;
    let t7973 = -F::new(8.0) / F::new(3.0) * t1419 * t56 - F::new(5.0) / F::new(6.0) * t7251 * t1409 + t6503;
    (t7771, t7773, t7774, t7973)
}
