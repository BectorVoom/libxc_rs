//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1059/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1059<F: Float>(t24260: F, t24280: F, t858: F, t23230: F, t225: F, t7072: F, t23198: F, t23206: F, t23209: F, t23220: F, t23224: F, t23232: F, t23235: F, t23239: F, t24200: F, t24235: F, t24237: F, t259: F, t2597: F, t2713: F, t2720: F, t7087: F, t7092: F, t7107: F, t855: F, t866: F) -> (F, F, F, F) {
    let t24281 = t24260 + t24280;
    let t24282 = t858 * t24281;
    let t24291 = F::cast_from(0.16449340668482264365e-1_f64) * t23230;
    let t24297 = t7072 * t225;
    let t24300 = t24200 * t259 + t24235 * t259 + F::new(2.0) * t24237 * t259 - F::new(2.0) * t2713 * t7107 - t855 * t24282 + F::cast_from(0.3289868133696452873e-1_f64) * t23198 + F::cast_from(0.3289868133696452873e-1_f64) * t23206 + F::cast_from(0.16449340668482264365e-1_f64) * t23209 - F::cast_from(0.16449340668482264365e-1_f64) * t23220 - F::cast_from(0.3289868133696452873e-1_f64) * t23224 + F::new(2.0) * t7087 * t2720 - t24291 + F::cast_from(0.15352717957250113407e0_f64) * t23232 + F::cast_from(0.76763589786250567036e-1_f64) * t23235 - F::cast_from(0.6579736267392905746e-1_f64) * t23239 + F::new(4.0) * t2597 * t7092 - F::new(2.0) * t24297 * t866;
    (t24281, t24282, t24297, t24300)
}
