//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1229/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1229<F: Float>(t4674: F, t94: F, t1689: F, t6103: F, t6106: F, t6243: F, t6246: F, t196: F, t197: F, t5322: F, t1779: F, t30: F, t4706: F, t1713: F, t1364: F, t1398: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21236 = t94 * t4674;
    let t21238 = 2.0 * t21236 * t1689;
    let t21240 = 4.0 * t6103 * t6106;
    let t21247 = 6.0 * t6243 * t6246;
    let t21253 = t5322 * t196 * t197;
    let t21254 = t21253 * t1779;
    let t21255 = t30 * t4706;
    let t21256 = t1713 * t21255;
    let t21262 = t1364 * t1398;
    (t21236, t21238, t21240, t21247, t21253, t21254, t21255, t21256, t21262)
}
