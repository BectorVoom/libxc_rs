//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2021/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2021<F: Float>(t91143: F, t91149: F, t91167: F, t91179: F, t93651: F, t93652: F, t93653: F, t93657: F, t97273: F, t97277: F, t97281: F, t97283: F, t97287: F, t97291: F, t97295: F, t97299: F, t97303: F, t97307: F) -> F {
    let t102679 = -F::cast_from(0.80745512188280781708e-3_f64) * t91143 + F::cast_from(0.48447307312968469024e-2_f64) * t97273 + F::cast_from(0.48447307312968469024e-2_f64) * t97277 - F::cast_from(0.48447307312968469024e-2_f64) * t97281 - F::new(119.0) / F::new(432.0) * t91149 + t93651 - t93652 + t93653 - F::new(35.0) / F::new(288.0) * t97283 - F::cast_from(0.16956557559538964158e-1_f64) * t97287 + F::cast_from(0.24223653656484234512e-2_f64) * t97291 + F::cast_from(0.24223653656484234512e-2_f64) * t97295 + F::cast_from(0.24223653656484234512e-2_f64) * t97299 - F::cast_from(0.80745512188280781706e-3_f64) * t97303 - F::cast_from(0.40372756094140390853e-3_f64) * t97307 - F::cast_from(0.45217486825437237755e-1_f64) * t91167 - t93657 - F::cast_from(0.23739180583354549821e0_f64) * t91179;
    t102679
}
