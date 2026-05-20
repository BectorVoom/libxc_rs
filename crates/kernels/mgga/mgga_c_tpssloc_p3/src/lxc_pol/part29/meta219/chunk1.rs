//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1052/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1052<F: Float>(t1174: F, t4916: F, t1714: F, t3448: F, t3451: F, t3295: F, t3464: F, t4770: F, t4773: F, t4776: F, t4779: F) -> (F, F, F, F) {
    let t4917 = t1174 * t4916;
    let t4919 = t3448 * t1714;
    let t4920 = t4919 * t3451;
    let t4928 = -t3464 + t3295 / F::new(9.0) + t4770 / F::new(9.0) + t4773 / F::new(18.0) - t4776 / F::new(3.0) - t4779 / F::new(6.0);
    (t4917, t4919, t4920, t4928)
}
