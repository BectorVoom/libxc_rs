//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2164/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2164<F: Float>(t81955: F, t81957: F, t81964: F, t87458: F, t87464: F, t87466: F, t87469: F, t87472: F, t87475: F, t87478: F, t87481: F, t87485: F, t87488: F, t87491: F, t87495: F, t87498: F, t87502: F, t87507: F) -> F {
    let t87509 = -F::cast_from(0.24223653656484234512e-2_f64) * t87458 - t81955 - F::new(7.0) / F::new(48.0) * t81957 - F::cast_from(0.59347951458386374556e-1_f64) * t81964 - t87464 + t87466 / F::new(8.0) + t87469 / F::new(16.0) - F::cast_from(0.20186378047070195427e-3_f64) * t87472 - F::cast_from(0.40372756094140390854e-3_f64) * t87475 - t87478 - F::cast_from(0.20186378047070195427e-3_f64) * t87481 + F::cast_from(0.12111826828242117256e-2_f64) * t87485 + t87488 + F::cast_from(0.12111826828242117256e-2_f64) * t87491 - F::cast_from(0.20186378047070195427e-3_f64) * t87495 + F::cast_from(0.80745512188280781708e-3_f64) * t87498 + F::cast_from(0.12111826828242117256e-2_f64) * t87502 - F::cast_from(0.48447307312968469024e-2_f64) * t87507;
    t87509
}
