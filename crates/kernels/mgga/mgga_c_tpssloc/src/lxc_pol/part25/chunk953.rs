//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 953/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk953<F: Float>(t12248: F, t236: F, t240: F, t1336: F, t12251: F, t1343: F, t820: F, t12255: F, t3777: F, t3798: F, t1354: F, t1307: F, t3719: F) -> (F, F, F, F, F, F) {
    let t12289 = t12248 * t236;
    let t12290 = t12289 * t240;
    let t12291 = t1336 * t12290;
    let t12293 = t1343 * t820 * t12251;
    let t12297 = t1343 * t820 * t12255;
    let t12300 = t3777 * t3798;
    let t12301 = t12300 * t1354;
    let t12303 = t1307 * t3719;
    (t12289, t12291, t12293, t12297, t12301, t12303)
}
