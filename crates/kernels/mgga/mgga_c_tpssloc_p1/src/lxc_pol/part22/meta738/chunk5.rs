//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2427/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2427<F: Float>(t42213: F, t47787: F, t59700: F, t59702: F, t59704: F, t60274: F, t68619: F, t68626: F, t68628: F, t68630: F, t68633: F, t68635: F) -> F {
    let t69143 = t42213 - F::cast_from(0.6618234375e1_f64) * t68619 + F::cast_from(0.69463333333333333333e-1_f64) * t60274 - F::cast_from(0.20658999999999999999e1_f64) * t59700 + F::cast_from(0.68863333333333333332e0_f64) * t59702 + F::cast_from(0.5738611111111111111e0_f64) * t59704 + F::cast_from(0.16068111111111111111e1_f64) * t47787 + F::cast_from(0.794188125e1_f64) * t68626 - F::cast_from(0.52945875e1_f64) * t68628 - F::cast_from(0.52945875e1_f64) * t68630 + F::cast_from(0.2366859375e0_f64) * t68633 - F::cast_from(0.473371875e0_f64) * t68635;
    t69143
}
