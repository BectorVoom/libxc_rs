//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1217/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1217<F: Float>(t225: F, t24237: F, t24235: F, t2047: F, t24305: F, t24330: F, t259: F, t2713: F, t2743: F, t7107: F, t82266: F, t82282: F, t82294: F, t82296: F, t866: F, t9584: F, t9590: F, t9593: F) -> F {
    let t85146 = t24237 * t225;
    let t85152 = t24235 * t225;
    let t85163 = F::cast_from(0.29608813203268075857e0_f64) * t82266 - F::cast_from(3.0_f64) * t24305 * t2743 - F::cast_from(6.0_f64) * t85146 * t866 - F::cast_from(6.0_f64) * t9593 * t7107 - F::cast_from(0.39478417604357434476e0_f64) * t82282 - F::cast_from(3.0_f64) * t85152 * t866 - F::cast_from(3.0_f64) * t9590 * t7107 + t9584 * t2047 * t259 - F::cast_from(0.31253747270116302294e0_f64) * t82294 - F::cast_from(0.69087230807625510332e0_f64) * t82296 + F::cast_from(6.0_f64) * t2713 * t24330;
    t85163
}
