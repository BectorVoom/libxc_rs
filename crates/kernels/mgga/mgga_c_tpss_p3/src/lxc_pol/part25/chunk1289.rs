//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1289/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1289<F: Float>(t236: F, t339: F, t60698: F, t18464: F, t4480: F, t1642: F, t60706: F, t18450: F, t4462: F, t60731: F, t4473: F, t60738: F) -> (F, F, F, F, F, F) {
    let t65607 = t339 * t60698 * t236;
    let t65616 = t18464 * t4480;
    let t65624 = t60706 * t1642;
    let t65628 = t18450 * t4462;
    let t65634 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t60731;
    let t65639 = t60738 * t4473;
    (t65607, t65616, t65624, t65628, t65634, t65639)
}
