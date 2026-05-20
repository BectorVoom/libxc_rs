//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1849/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1849<F: Float>(t225: F, t26732: F, t87776: F, t87786: F, t87796: F, t87804: F, t87835: F, t87873: F, t26734: F, t87901: F, t87910: F, t87927: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t92847 = t26732 * t225;
    let t92862 = F::cast_from(0.16449340668482264365e-1_f64) * t87776;
    let t92866 = F::cast_from(0.15352717957250113407e0_f64) * t87786;
    let t92872 = F::cast_from(0.76763589786250567036e-1_f64) * t87796;
    let t92874 = F::cast_from(0.76763589786250567036e-1_f64) * t87804;
    let t92910 = F::cast_from(0.3289868133696452873e-1_f64) * t87835;
    let t92938 = F::cast_from(0.3289868133696452873e-1_f64) * t87873;
    let t92939 = t26734 * t225;
    let t92955 = F::cast_from(0.3289868133696452873e-1_f64) * t87901;
    let t92960 = F::cast_from(0.16449340668482264365e-1_f64) * t87910;
    let t92966 = F::cast_from(0.9869604401089358619e-1_f64) * t87927;
    (t92847, t92862, t92866, t92872, t92874, t92910, t92938, t92939, t92955, t92960, t92966)
}
