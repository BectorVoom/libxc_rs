//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1364/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1364<F: Float>(t3255: F, t5918: F, t65551: F, t65561: F, t65567: F, t60696: F, t60707: F, t60709: F, t60713: F, t62375: F, t65553: F, t65555: F, t65557: F, t65559: F) -> (F, F) {
    let t67131 = t3255 * t5918;
    let t67138 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t65551;
    let t67143 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t65561;
    let t67148 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t65567;
    let t67149 = t67138 + t65553 / F::cast_from(96.0_f64) - F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t65555 - t65557 / F::cast_from(48.0_f64) - t62375 + t65559 / F::cast_from(384.0_f64) - t67143 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t60696 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t60707 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t60709 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t60713 - t67148;
    (t67131, t67149)
}
