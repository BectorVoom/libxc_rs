//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 894/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk894<F: Float>(t21126: F, t908: F, t136: F, t21122: F, t2826: F, t10577: F, t13598: F, t17149: F, t17165: F, t17175: F, t21124: F, t21128: F, t21147: F, t21150: F, t21153: F, t21156: F) -> (F, F, F) {
    let t21160 = t908 * t21126;
    let t21161 = t136 * t21160;
    let t21167 = t2826 * t21122;
    let t21168 = t136 * t21167;
    let t21180 = -t10577 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13598 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17149 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t17165 + t17175 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t21147 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t21150 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t21124 - F::cast_from(2.0_f64) * t21153 + F::cast_from(2.0_f64) * t21128 - t21156 / F::cast_from(3.0_f64);
    (t21161, t21168, t21180)
}
