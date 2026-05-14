//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 935/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk935<F: Float>(t1206: F, t3260: F, t1220: F, t339: F, t790: F, t3346: F, t72: F, t240: F, t3243: F, t756: F, t1246: F, t159: F, t210: F, t1212: F, t2139: F, t1215: F) -> (F, F, F, F, F, F, F, F) {
    let t10106 = t3260 * t1206;
    let t10117 = t339 * t1220 * t790;
    let t10120 = t3346 * t72;
    let t10121 = t10120 * t240;
    let t10137 = t756 * t3243;
    let t10140 = t159 * t1246;
    let t10141 = t210 * t10140;
    let t10160 = t2139 * t1212;
    let t10161 = t10160 * t1215;
    (t10106, t10117, t10120, t10121, t10137, t10141, t10160, t10161)
}
