//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 806/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk806<F: Float>(t5: F, t32244: F, t9239: F, t33: F, t8705: F, t2240: F, t20: F, t60: F, t131: F, t8308: F, t8302: F, t31000: F, t31006: F, t31013: F, t31024: F, t8707: F, t112: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t32245 = t9239 * t32244;
    let t32248 = t33 * t8705;
    let t32249 = t2240 * t32248;
    let t32253 = 1.0 / t60 / t20;
    let t32255 = t32253 * t131 * t8308;
    let t32257 = 20.0 / 27.0 * t8302 * t32255;
    let t32258 = t2240 * t32244;
    let t32262 = piecewise3(t8, 0.0, 5.0 / 36.0 * t31000 * t8707 - 5.0 / 6.0 * t32245 * t31006 - 5.0 / 9.0 * t32249 * t31013 - t32257 + 5.0 / 18.0 * t32258 * t31024);
    let t32263 = t32262 * t112;
    (t32245, t32248, t32249, t32253, t32255, t32257, t32258, t32262, t32263)
}
