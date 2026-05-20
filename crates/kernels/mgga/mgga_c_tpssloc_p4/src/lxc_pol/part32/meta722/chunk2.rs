//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2301/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2301<F: Float>(t103345: F, t2122: F, t24574: F, t29674: F, t29750: F, t85853: F, t1011: F, t6218: F, t225: F, t29624: F, t29614: F, t103223: F, t19189: F, t24589: F, t24788: F, t24812: F, t24833: F, t27461: F, t27473: F, t27489: F, t27516: F, t27553: F, t29740: F, t29744: F, t4978: F, t7364: F, t7373: F, t7375: F, t7376: F, t94784: F, t94787: F) -> (F, F, F, F) {
    let t103490 = t2122 * t103345;
    let t103494 = t24574 * t29674;
    let t103507 = t85853 * t29750;
    let t103515 = t6218 * t1011;
    let t103520 = t29624 * t225;
    let t103528 = t29614 * t225;
    let t103538 = F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t27516 * t27473 + F::cast_from(0.54831135561607547883e-2_f64) * t103507 - F::cast_from(0.36554090374405031923e-2_f64) * t94784 - F::cast_from(0.19495514866349350359e-1_f64) * t103223 * t27553 + t94787 - F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t24833 * t29744 + F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t27489 * t103515 * t4978 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t103520 * t7364 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t7375 * t19189 * t7376 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t103528 * t7364 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24788 * t29740 + F::cast_from(0.54831135561607547883e-2_f64) * t24589 * t27516 * t27461;
    (t103490, t103494, t103515, t103538)
}
