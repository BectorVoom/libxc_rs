//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 435/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk435<F: Float>(t3071: F, t3072: F, t225: F, t3020: F, t68: F, t369: F, t374: F, t376: F, t677: F, t370: F, t35: F, t365: F, t612: F) -> (F, F, F, F, F) {
    let t3073 = t3071 * t3072;
    let t3076 = t3020 * t225;
    let t3077 = t3076 * t68;
    let t3078 = t3077 * t369;
    let t3082 = t374 * t677 * t376;
    let t3084 = t370 * t3082 / F::cast_from(13824.0_f64);
    let t3087 = F::cast_from(1.0_f64) / t35 / t365 / t612;
    (t3073, t3076, t3078, t3084, t3087)
}
