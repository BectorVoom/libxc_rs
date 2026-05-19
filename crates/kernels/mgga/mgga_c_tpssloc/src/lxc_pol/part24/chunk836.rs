//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 836/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk836<F: Float>(t52: F, t607: F, t78: F, t2250: F, t638: F, t771: F, t9258: F, t9288: F, t9505: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t9508 = t78 * t607;
    let t9514 = piecewise3::<F>(t150, F::new(0.0), -F::new(8.0) / F::new(27.0) * t638 * t9288 - F::new(2.0) / F::new(3.0) * t9508 * t2250 - F::new(2.0) / F::new(3.0) * t771 * t9258);
    let t9516 = t9505 / F::new(2.0) + t9514 / F::new(2.0);
    t9516
}
