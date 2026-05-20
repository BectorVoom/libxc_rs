//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1215/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1215<F: Float>(t12282: F, t1336: F, t3777: F, t3789: F, t12248: F, t236: F, t3798: F, t12189: F, t1329: F, t1333: F, t3862: F, t10022: F, t248: F, t557: F) -> (F, F, F, F, F, F, F) {
    let t12283 = t1336 * t12282;
    let t12286 = t3777 * t3789;
    let t12289 = t12248 * t236;
    let t12300 = t3777 * t3798;
    let t12308 = t12189 * t1329;
    let t12325 = t1333 * t3862;
    let t12328 = t10022 * t557 * t248;
    (t12283, t12286, t12289, t12300, t12308, t12325, t12328)
}
