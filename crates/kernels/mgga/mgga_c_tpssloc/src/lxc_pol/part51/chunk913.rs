//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 913/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk913<F: Float>(t25146: F, t831: F, t1512: F, t23053: F, t4236: F, t6614: F, t1878: F, t23033: F, t221: F, t4255: F, t23125: F, t23134: F, t23141: F, t23144: F, t25140: F, t25142: F, t25144: F) -> (F, F, F, F, F) {
    let t25147 = t25146 * t831;
    let t25149 = t23053 * t1512;
    let t25151 = t6614 * t4236;
    let t25154 = t1878 * t23033;
    let t25155 = t221 * t4255;
    let t25156 = t25154 * t25155;
    let t25158 = 0.20186378047070195427e-3 * t23125 + 7.0 / 144.0 * t25140 + 5.0 / 384.0 * t25142 + 7.0 / 2304.0 * t25144 - t25147 / 1536.0 - t25149 / 1536.0 - t25151 / 1536.0 + 7.0 / 576.0 * t23134 + t23141 + t23144 + t25156 / 16.0;
    (t25147, t25149, t25151, t25156, t25158)
}
