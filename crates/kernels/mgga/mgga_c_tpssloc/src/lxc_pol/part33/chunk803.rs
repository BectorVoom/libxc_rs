//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 803/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk803<F: Float>(t761: F, t9713: F, t177: F, t2508: F, t2512: F, t9490: F, t2475: F, t723: F, t159: F, t2461: F, t730: F, t167: F, t2478: F) -> (F, F, F, F, F, F, F) {
    let t9715 = F::cast_from(0.5848223622634646207e0_f64) * t761 * t9713;
    let t9720 = F::new(1.0) / t2508 / t177;
    let t9722 = t9720 * t9490 * t2512;
    let t9724 = F::cast_from(0.10389515463408878255e3_f64) * t761 * t9722;
    let t9729 = F::new(1.0) / t2475 / t723;
    let t9730 = t159 * t9729;
    let t9731 = t2461 * t730;
    let t9733 = F::new(1.0) / t2478 / t167;
    (t9715, t9720, t9722, t9724, t9730, t9731, t9733)
}
