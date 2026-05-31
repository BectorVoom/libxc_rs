//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 770/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk770<F: Float>(t40: F, t52: F, t5392: F, t5398: F, t75: F, t767: F, t771: F, t78: F, zeta_threshold: F) -> F {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t5536 = piecewise3::<F>(t146, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t75 * t5392 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t767 * t5398);
    let t5542 = piecewise3::<F>(t150, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t78 * t5392 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t771 * t5398);
    let t5544 = t5536 / F::cast_from(2.0_f64) + t5542 / F::cast_from(2.0_f64);
    t5544
}
