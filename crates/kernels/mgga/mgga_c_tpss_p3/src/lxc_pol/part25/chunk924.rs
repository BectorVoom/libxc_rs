//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 924/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk924<F: Float>(t1246: F, t159: F, t210: F, t1212: F, t2139: F, t1215: F, t242: F, t527: F, t8200: F, t525: F, t1257: F, t73: F) -> (F, F, F, F, F, F, F, F) {
    let t10140 = t159 * t1246;
    let t10141 = t210 * t10140;
    let t10160 = t2139 * t1212;
    let t10161 = t10160 * t1215;
    let t10164 = t8200 * t527 * t242;
    let t10166 = F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t525 * t10164;
    let t10178 = t1257 * t1257;
    let t10179 = F::cast_from(1.0_f64) / t10178;
    let t10180 = t73 * t10179;
    (t10141, t10160, t10161, t10164, t10166, t10178, t10179, t10180)
}
