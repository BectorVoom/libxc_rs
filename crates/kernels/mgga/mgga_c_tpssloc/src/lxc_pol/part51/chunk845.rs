//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 845/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk845<F: Float>(t7181: F, t7183: F, t7185: F, t7189: F, t7706: F, t7710: F, t7713: F, t7716: F, t7718: F, t7720: F) -> F {
    let t7918 = -t7181 - t7706 / F::new(24.0) - t7183 - F::cast_from(0.24223653656484234512e-2_f64) * t7710 - t7185 - F::cast_from(0.40372756094140390853e-3_f64) * t7713 + t7716 / F::new(768.0) - t7718 / F::new(768.0) - t7189 - t7720 / F::new(192.0);
    t7918
}
