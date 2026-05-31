//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 973/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk973<F: Float>(t40: F, t52: F, t20217: F, t20234: F, t4080: F, t5398: F, t73: F, t9427: F, t4087: F, t76: F, t9438: F, t157: F, t182: F, t16587: F, zeta_threshold: F) -> (F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t20732 = piecewise3::<F>(t146, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9427 * t20234 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4080 * t5398 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t73 * t20217);
    let t20740 = piecewise3::<F>(t150, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9438 * t20234 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4087 * t5398 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t76 * t20217);
    let t20741 = t20732 + t20740;
    let t20742 = t20741 * t157;
    let t20744 = F::cast_from(0.19751673498613801407e-1_f64) * t20742 * t182;
    let t20745 = F::cast_from(36.0_f64) * t16587;
    (t20741, t20744, t20745)
}
