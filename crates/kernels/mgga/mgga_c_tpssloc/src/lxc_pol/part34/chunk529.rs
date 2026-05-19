//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 529/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk529<F: Float>(t52: F, t2440: F, t5392: F, t5398: F, t76: F, t5512: F, t145: F, t185: F, t157: F, t182: F, t4200: F, t2373: F, t2377: F, t2408: F, t2417: F, t2522: F, t5497: F, t5498: F, t5501: F, t5502: F, t5506: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t150 = t52 <= zeta_threshold;
    let t5518 = piecewise3::<F>(t150, F::new(0.0), F::new(4.0) / F::new(9.0) * t2440 * t5392 - F::new(4.0) / F::new(3.0) * t76 * t5398);
    let t5519 = t5512 + t5518;
    let t5520 = t145 * t5519;
    let t5521 = t5520 * t185;
    let t5522 = t5519 * t157;
    let t5524 = F::cast_from(0.19751673498613801407e-1_f64) * t5522 * t182;
    let t5525 = F::cast_from(0.11696447245269292414e1_f64) * t4200;
    let t5526 = F::new(6.0) * t2522 * t5502 + t2373 + t2377 + t2408 + t2417 + t5497 + t5498 + t5501 + t5506 + t5521 + t5524 - t5525;
    (t5519, t5520, t5521, t5522, t5524, t5525, t5526)
}
