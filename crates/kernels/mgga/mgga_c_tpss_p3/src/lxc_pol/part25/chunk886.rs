//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 886/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk886<F: Float>(t2146: F, t756: F, t159: F, t799: F, t210: F, t2139: F, t760: F, t764: F, t64: F, t7091: F, t216: F, t570: F, t66: F) -> (F, F, F, F, F, F, F) {
    let t8167 = t756 * t2146;
    let t8170 = t159 * t799;
    let t8171 = t210 * t8170;
    let t8176 = t2139 * t760;
    let t8177 = t8176 * t764;
    let t8185 = t64 * t7091;
    let t8186 = t8185 * t159;
    let t8188 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t8186 * t216;
    let t8199 = F::cast_from(1.0_f64) / t66 / t570;
    (t8167, t8171, t8176, t8177, t8186, t8188, t8199)
}
