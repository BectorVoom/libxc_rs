//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 957/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk957<F: Float>(t114688: F, t114693: F, t225: F, t31985: F, t131: F, t32248: F, t9239: F, t2240: F, t32253: F, t33: F, t31013: F, t8302: F, t8308: F, t9533: F) -> (F, F, F, F, F, F, F, F) {
    let t116686 = F::cast_from(0.3289868133696452873e-1_f64) * t114688;
    let t116688 = F::cast_from(0.25587863262083522346e0_f64) * t114693;
    let t116709 = t31985 * t225;
    let t116904 = t32248 * t131;
    let t116905 = t9239 * t116904;
    let t116909 = t2240 * t33 * t32253;
    let t116910 = t116909 * t31013;
    let t116917 = F::new(380.0) / F::new(81.0) * t8302 * t9533 * t131 * t8308;
    (t116686, t116688, t116709, t116904, t116905, t116909, t116910, t116917)
}
