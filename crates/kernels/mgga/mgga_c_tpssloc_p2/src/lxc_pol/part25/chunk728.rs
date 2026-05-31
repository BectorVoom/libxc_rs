//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 728/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk728<F: Float>(t40: F, t180: F, t2511: F, t9489: F, t9490: F, t761: F, t607: F, t75: F, t2250: F, t634: F, t767: F, t9258: F, t9288: F, zeta_threshold: F) -> (F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t9493 = F::cast_from(1.0_f64) / t2511 / t180;
    let t9494 = t9489 * t9490 * t9493;
    let t9496 = F::cast_from(0.10254018858216406658e4_f64) * t761 * t9494;
    let t9499 = t75 * t607;
    let t9505 = piecewise3::<F>(t146, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t634 * t9288 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9499 * t2250 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t767 * t9258);
    (t9493, t9494, t9496, t9505)
}
