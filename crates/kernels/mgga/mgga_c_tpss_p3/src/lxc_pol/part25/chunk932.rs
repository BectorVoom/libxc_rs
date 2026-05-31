//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 932/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk932<F: Float>(t3638: F, t8313: F, t236: F, t339: F, t8276: F, t3678: F, t219: F, t3693: F, t220: F, t73: F, t8275: F, t3692: F, t768: F) -> (F, F, F, F, F, F) {
    let t10777 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t8313 * t3638;
    let t10779 = t339 * t8276 * t236;
    let t10803 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t8313 * t3678;
    let t10821 = t3693 * t219;
    let t10845 = t220 * t73 * t8275;
    let t10884 = t768 * t3692;
    (t10777, t10779, t10803, t10821, t10845, t10884)
}
