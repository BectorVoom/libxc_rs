//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 557/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk557<F: Float>(t182: F, t5522: F, t4200: F, t2373: F, t2377: F, t2408: F, t2417: F, t2522: F, t5497: F, t5498: F, t5501: F, t5502: F, t5506: F, t5521: F, t1484: F) -> (F, F, F, F) {
    let t5524 = 0.19751673498613801407e-1 * t5522 * t182;
    let t5525 = 0.11696447245269292414e1 * t4200;
    let t5526 = 6.0 * t2522 * t5502 + t2373 + t2377 + t2408 + t2417 + t5497 + t5498 + t5501 + t5506 + t5521 + t5524 - t5525;
    let t5527 = t1484 * t1484;
    (t5524, t5525, t5526, t5527)
}
