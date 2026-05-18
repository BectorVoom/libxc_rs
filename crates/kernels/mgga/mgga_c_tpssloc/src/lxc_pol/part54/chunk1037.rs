//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1037/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1037<F: Float>(t1375: F, t16460: F, t2016: F, t26224: F, t26226: F, t26229: F, t26329: F, t26335: F, t26340: F, t26345: F, t26348: F, t26352: F, t26357: F, t26361: F, t3882: F, t5321: F, t568: F, t6963: F, t7729: F) -> F {
    let t26364 = -F::new(6.0) * t26224 * t26226 + t26229 * t568 + t26329 * t568 + F::new(0.49348022005446793095e-1) * t26335 + F::new(0.16449340668482264365e-1) * t26340 + F::new(2.0) * t3882 * t7729 + F::new(0.41123351671205660912e-2) * t26345 + F::new(2.0) * t1375 * t26348 - F::new(0.82246703342411321825e-2) * t26352 + F::new(0.16449340668482264365e-1) * t26357 + F::new(2.0) * t5321 * t6963 - F::new(0.19190897446562641759e-1) * t26361 - t16460 * t2016;
    t26364
}
