//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 978/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk978<F: Float>(t25: F, t12052: F, t522: F, t9216: F, t9218: F, t1294: F, t9713: F, t526: F, t3664: F, t606: F, t11988: F, t2249: F, t514: F, t9257: F, t28: F, t528: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t12053 = 36.0 * t12052;
    let t12054 = t9216 * t522;
    let t12055 = 240.0 * t12054;
    let t12057 = 120.0 * t9218 * t522;
    let t12059 = 0.5848223622634646207e0 * t1294 * t9713;
    let t12061 = 1.0 / t526 / t25;
    let t12064 = t3664 * t606;
    let t12070 = piecewise3(t26, 0.0, -8.0 / 27.0 * t12061 * t11988 + 4.0 / 3.0 * t12064 * t2249 + 4.0 / 3.0 * t514 * t9257);
    let t12072 = 1.0 / t528 / t28;
    (t12053, t12055, t12057, t12059, t12070, t12072)
}
