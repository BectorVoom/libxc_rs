//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 869/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk869<F: Float>(t3850: F, t562: F, t1352: F, t12240: F, t3806: F, t5248: F, t1339: F, t836: F, t1336: F, t3809: F, t3777: F, t3789: F, t12248: F, t236: F, t240: F, t12251: F, t1343: F, t820: F) -> (F, F, F, F, F, F, F, F) {
    let t12272 = t562 * t3850;
    let t12273 = t12272 * t1352;
    let t12279 = t5248 * t3806 * t12240;
    let t12282 = t1339 * t836;
    let t12283 = t1336 * t12282;
    let t12284 = t12283 * t3809;
    let t12286 = t3777 * t3789;
    let t12289 = t12248 * t236;
    let t12290 = t12289 * t240;
    let t12291 = t1336 * t12290;
    let t12293 = t1343 * t820 * t12251;
    (t12272, t12273, t12279, t12284, t12286, t12289, t12291, t12293)
}
