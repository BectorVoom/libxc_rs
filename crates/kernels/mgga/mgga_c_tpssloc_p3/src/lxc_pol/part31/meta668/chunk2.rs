//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1967/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1967<F: Float>(t84857: F, t84859: F, t87213: F, t92580: F, t92582: F, t98647: F, t98651: F, t98655: F, t98659: F, t98663: F, t98668: F, t98672: F, t98674: F, t98676: F, t98678: F, t98680: F, t98682: F, t98685: F) -> F {
    let t101413 = F::cast_from(0.40372756094140390853e-3_f64) * t98647 - t92580 - t84857 + t84859 + F::cast_from(0.24223653656484234512e-2_f64) * t98651 - F::cast_from(0.80745512188280781706e-3_f64) * t98655 - F::cast_from(0.40372756094140390853e-3_f64) * t98659 + F::cast_from(0.24223653656484234512e-2_f64) * t98663 + F::cast_from(0.48447307312968469024e-2_f64) * t98668 + F::cast_from(0.48447307312968469024e-2_f64) * t98672 - F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t98674 + t98676 / F::cast_from(96.0_f64) - t98678 / F::cast_from(384.0_f64) - t98680 / F::cast_from(768.0_f64) - t98682 / F::cast_from(768.0_f64) - t98685 / F::cast_from(768.0_f64) + t92582 + F::cast_from(0.6728792682356731809e-4_f64) * t87213;
    t101413
}
