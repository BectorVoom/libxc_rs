//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1422/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1422<F: Float>(t102466: F, t120340: F, t120436: F, t120533: F, t122260: F, t122270: F, t122278: F, t122281: F, t16022: F, t16460: F, t26224: F, t26482: F, t31555: F, t31653: F, t5321: F, t5326: F, t6962: F, t7194: F, t8627: F) -> F {
    let t122285 = -t120340 - F::new(0.82246703342411321825e-2) * t122260 + F::new(2.0) * t16022 * t8627 - t120436 - F::new(6.0) * t26224 * t102466 * t6962 - t120533 + F::new(2.0) * t31653 * t5326 + F::new(0.16449340668482264365e-1) * t122270 + F::new(2.0) * t5321 * t31555 + F::new(2.0) * t16460 * t8627 + F::new(0.16449340668482264365e-1) * t122278 - F::new(0.82246703342411321825e-2) * t122281 + F::new(2.0) * t7194 * t26482;
    t122285
}
