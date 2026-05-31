//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 720/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk720<F: Float>(t3: F, t8811: F, t2039: F, t3941: F, t577: F, t7230: F, t8508: F, t8717: F, t192: F, t533: F, t89: F) -> (F, F, F, F) {
    let t8812 = t3 * t8811;
    let t8822 = F::cast_from(0.45e1_f64) * t8811 * t577 + F::cast_from(27.0_f64) * t7230 * t2039 + F::cast_from(27.0_f64) * t3941 * t8717 + t8508;
    let t8944 = t192 * t533;
    let t9003 = t89 * t2039;
    (t8812, t8822, t8944, t9003)
}
