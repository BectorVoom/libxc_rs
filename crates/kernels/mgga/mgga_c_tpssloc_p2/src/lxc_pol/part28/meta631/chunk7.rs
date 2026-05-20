//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1984/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1984<F: Float>(t87463: F, t87477: F, t87487: F, t81957: F, t81964: F, t84932: F, t87458: F, t87466: F, t87469: F, t87472: F, t87475: F, t87481: F, t87485: F, t87491: F, t87495: F, t87498: F, t87502: F, t87507: F) -> F {
    let t92705 = F::new(7.0) / F::new(12.0) * t87463;
    let t92710 = F::cast_from(0.33913115119077928316e-1_f64) * t87477;
    let t92713 = F::cast_from(0.56521858531796547194e-2_f64) * t87487;
    let t92719 = -F::cast_from(0.48447307312968469024e-2_f64) * t87458 - t84932 - F::new(7.0) / F::new(24.0) * t81957 - F::cast_from(0.11869590291677274911e0_f64) * t81964 - t92705 + t87466 / F::new(4.0) + t87469 / F::new(8.0) - F::cast_from(0.40372756094140390853e-3_f64) * t87472 - F::cast_from(0.80745512188280781706e-3_f64) * t87475 - t92710 - F::cast_from(0.40372756094140390853e-3_f64) * t87481 + F::cast_from(0.24223653656484234512e-2_f64) * t87485 + t92713 + F::cast_from(0.24223653656484234512e-2_f64) * t87491 - F::cast_from(0.40372756094140390853e-3_f64) * t87495 + F::cast_from(0.16149102437656156341e-2_f64) * t87498 + F::cast_from(0.24223653656484234512e-2_f64) * t87502 - F::cast_from(0.96894614625936938048e-2_f64) * t87507;
    t92719
}
