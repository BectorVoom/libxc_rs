//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 230/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk230<F: Float>(t57: F, t581: F, t745: F, t744: F, zeta_threshold: F) -> F {
    let t155 = t57 <= zeta_threshold;
    let t748 = piecewise3::<F>(t155, F::new(0.0), -F::new(2.0) / F::new(3.0) * t745 * t581);
    let t750 = t744 / F::new(2.0) + t748 / F::new(2.0);
    t750
}
