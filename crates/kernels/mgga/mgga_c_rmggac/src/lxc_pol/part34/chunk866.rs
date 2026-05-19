//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 866/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk866<F: Float>(t118: F, t1986: F, t615: F, t665: F, t7717: F, t2046: F, t2049: F, t2323: F, t15039: F, t2160: F, t638: F, t14286: F, t551: F) -> (F, F, F, F) {
    let t75498 = t1986 * t118 * t665 * t615;
    let t75500 = F::cast_from(0.1064114997332445985e-4_f64) * t7717 * t75498;
    let t75508 = t2046 * t2049 * t2323;
    let t75513 = t638 * t2160 * t15039;
    let t75515 = t14286 * t551;
    (t75500, t75508, t75513, t75515)
}
