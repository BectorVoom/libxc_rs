//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 189/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk189<F: Float>(t600: F, t44: F, t49: F, t56: F, t589: F, t592: F, t595: F) -> (F, F) {
    let t601 = F::new(8.0) / F::new(3.0) * t600;
    let t602 = -F::new(8.0) / F::new(3.0) * t589 * t49 + F::new(5.0) / F::new(6.0) * t44 * t592 - F::new(5.0) / F::new(6.0) * t56 * t595 + t601;
    (t601, t602)
}
