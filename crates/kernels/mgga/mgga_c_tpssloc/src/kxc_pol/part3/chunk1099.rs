//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1099/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1099<F: Float>(t16048: F, t5248: F, t5249: F, t12283: F, t5259: F, t5293: F, t120: F, t5286: F, t3805: F, t3807: F, t12407: F, t12284: F, t12301: F, t12397: F, t12429: F, t1341: F, t1363: F, t16147: F, t16150: F, t16155: F, t16159: F, t16208: F, t16211: F, t16214: F, t16217: F, t16227: F, t16233: F, t1827: F, t3778: F, t3803: F, t5289: F) -> (F, F) {
    let t16235 = t5248 * t5249 * t16048;
    let t16239 = 7.0 / 576.0 * t12283 * t5259;
    let t16241 = 7.0 / 2304.0 * t12283 * t5293;
    let t16242 = t120 * t5286;
    let t16244 = t3805 * t16242 * t3807;
    let t16248 = t3805 * t5249 * t12407;
    let t16253 = -t16147 + 5.0 / 384.0 * t1363 * t16150 + 5.0 / 768.0 * t1363 * t16155 + t16159 - t1341 * t16208 / 3072.0 - 119.0 / 13824.0 * t16211 + t16214 - 5.0 / 128.0 * t1363 * t16217 - t12397 * t1827 / 3072.0 - t3778 * t5289 / 1536.0 - 5.0 / 384.0 * t3803 * t16227 - 7.0 / 576.0 * t12284 + 7.0 / 2304.0 * t12301 - t16233 * t16235 / 512.0 - t16239 + t16241 + t3803 * t16244 / 384.0 + t3803 * t16248 / 768.0 + t12429 * t5259 / 384.0;
    (t16242, t16253)
}
