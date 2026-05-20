//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 825/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk825<F: Float>(t9689: F, t9692: F, t9695: F, t9698: F, t9702: F, t9704: F, t9706: F, t9709: F, t739: F, t746: F, t761: F, t172: F, t2448: F) -> (F, F, F, F) {
    let t9711 = -F::cast_from(0.34523333333333333333e1_f64) * t9689 + F::cast_from(0.23015555555555555556e1_f64) * t9692 - F::cast_from(0.26851481481481481482e1_f64) * t9695 - F::cast_from(0.93932222222222222223e0_f64) * t9698 + F::new(0.73355e-1) * t9702 - F::new(0.14671e0) * t9704 - F::cast_from(0.17116166666666666667e0_f64) * t9706 - F::cast_from(0.36793333333333333333e0_f64) * t9709;
    let t9713 = t739 * t9711 * t746;
    let t9715 = F::cast_from(0.5848223622634646207e0_f64) * t761 * t9713;
    let t9716 = t2448 * t172;
    (t9711, t9713, t9715, t9716)
}
