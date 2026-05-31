//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1259/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1259<F: Float>(t5: F, t21784: F, t117: F, t4525: F, t6436: F, t18934: F, t18943: F, t19466: F, t19479: F, t19491: F, t21036: F, t21038: F, t21040: F, t21042: F, t21044: F, t21046: F, t21048: F, t21050: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t21785 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t21784);
    let t21786 = t21785 * t117;
    let t21790 = t6436 * t4525;
    let t21804 = t18934 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t19466 + t21036 / F::cast_from(8.0_f64) - t21038 / F::cast_from(24.0_f64) + t21040 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t19479 + t21042 / F::cast_from(96.0_f64) - t21044 / F::cast_from(768.0_f64) - t21046 / F::cast_from(768.0_f64) + t18943 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t19491 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t21048 - t21050 / F::cast_from(192.0_f64);
    (t21785, t21786, t21790, t21804)
}
