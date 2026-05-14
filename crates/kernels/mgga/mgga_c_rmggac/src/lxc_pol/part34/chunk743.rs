//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 743/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk743<F: Float>(t15382: F, t1971: F, t2144: F, t333: F, t7230: F, t352: F, t875: F, t118: F, t1986: F, t615: F, t665: F, t7717: F, t2046: F, t2049: F, t2323: F, t15039: F, t2160: F, t638: F) -> (F, F, F, F, F) {
    let t75490 = 0.3192344991997337955e-4 * t7230 * t1971 * t2144 * t15382 * t333;
    let t75495 = 0.212822999466489197e-4 * t7230 * t1971 * t875 * t15382 * t352;
    let t75498 = t1986 * t118 * t665 * t615;
    let t75500 = 0.1064114997332445985e-4 * t7717 * t75498;
    let t75508 = t2046 * t2049 * t2323;
    let t75513 = t638 * t2160 * t15039;
    (t75490, t75495, t75500, t75508, t75513)
}
