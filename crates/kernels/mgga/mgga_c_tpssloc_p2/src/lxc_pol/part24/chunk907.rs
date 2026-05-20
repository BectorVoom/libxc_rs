//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 907/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk907<F: Float>(t10321: F, t908: F, t136: F, t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10311: F, t10314: F, t10318: F, t10320: F) -> (F, F) {
    let t10322 = t908 * t10321;
    let t10323 = t136 * t10322;
    let t10325 = t10295 + F::new(5.0) / F::new(9.0) * t10296 - t10298 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t10300 - t10302 / F::new(3.0) + F::new(2.0) / F::new(27.0) * t10307 - t10311 / F::new(3.0) + t10314 / F::new(6.0) + t10318 - t10320 + t10323 / F::new(6.0);
    (t10323, t10325)
}
