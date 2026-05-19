//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1259/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1259<F: Float>(t5: F, t21784: F, t117: F, t4525: F, t6436: F, t18934: F, t18943: F, t19466: F, t19479: F, t19491: F, t21036: F, t21038: F, t21040: F, t21042: F, t21044: F, t21046: F, t21048: F, t21050: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t21785 = piecewise3::<F>(t8, F::new(0.0), t21784);
    let t21786 = t21785 * t117;
    let t21790 = t6436 * t4525;
    let t21804 = t18934 + F::new(7.0) / F::new(36.0) * t19466 + t21036 / F::new(8.0) - t21038 / F::new(24.0) + t21040 / F::new(384.0) + F::new(7.0) / F::new(576.0) * t19479 + t21042 / F::new(96.0) - t21044 / F::new(768.0) - t21046 / F::new(768.0) + t18943 + F::new(7.0) / F::new(144.0) * t19491 + F::new(5.0) / F::new(192.0) * t21048 - t21050 / F::new(192.0);
    (t21785, t21786, t21790, t21804)
}
