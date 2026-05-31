//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 999/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk999<F: Float>(t4212: F, t185: F, t5398: F, t707: F, t2373: F, t2377: F, t2408: F, t2417: F, t2423: F, t2426: F, t2486: F, t2518: F, t2530: F, t2537: F, t2665: F, t5497: F, t5498: F, t5501: F, t5506: F, t5521: F, t5524: F, t5525: F) -> (F, F, F, F) {
    let t5596 = F::cast_from(0.36622894612013090108e-3_f64) * t4212;
    let t5597 = t185 * t5398;
    let t5599 = F::cast_from(4.0_f64) * t707 * t5597;
    let t5600 = t2373 + t5524 + t5521 + t5498 + t2377 + t5497 - t2486 - t5596 - t5525 + t5506 + t2518 + t2408 + t2417 + t5501 - t2530 - t2537 - t2426 + t2665 - t2423 + t5599;
    (t5596, t5597, t5599, t5600)
}
