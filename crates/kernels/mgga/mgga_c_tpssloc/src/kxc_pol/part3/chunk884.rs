//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 884/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk884<F: Float>(t761: F, t9713: F, t172: F, t2448: F, t763: F, t177: F, t2508: F, t2512: F, t9490: F, t2517: F, t718: F, t2475: F, t723: F) -> (F, F, F, F, F, F, F) {
    let t9715 = F::cast_from(0.5848223622634646207e0_f64) * t761 * t9713;
    let t9716 = t2448 * t172;
    let t9717 = t9716 * t763;
    let t9720 = F::cast_from(1.0_f64) / t2508 / t177;
    let t9722 = t9720 * t9490 * t2512;
    let t9724 = F::cast_from(0.10389515463408878255e3_f64) * t761 * t9722;
    let t9726 = t718 * t2517;
    let t9729 = F::cast_from(1.0_f64) / t2475 / t723;
    (t9715, t9717, t9720, t9722, t9724, t9726, t9729)
}
