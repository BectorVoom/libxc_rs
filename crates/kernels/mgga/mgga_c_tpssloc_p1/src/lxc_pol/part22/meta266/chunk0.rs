//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1409/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1409<F: Float>(t12052: F, t522: F, t9216: F, t9218: F, t1294: F, t9713: F, t25: F, t526: F, t28: F, t528: F, t9722: F, t2528: F, t3691: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12053 = F::new(36.0) * t12052;
    let t12054 = t9216 * t522;
    let t12055 = F::new(240.0) * t12054;
    let t12057 = F::new(120.0) * t9218 * t522;
    let t12059 = F::cast_from(0.5848223622634646207e0_f64) * t1294 * t9713;
    let t12061 = F::new(1.0) / t526 / t25;
    let t12072 = F::new(1.0) / t528 / t28;
    let t12087 = F::cast_from(0.10389515463408878255e3_f64) * t1294 * t9722;
    let t12091 = t3691 * t2528;
    (t12053, t12054, t12055, t12057, t12059, t12061, t12072, t12087, t12091)
}
