//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 851/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk851<F: Float>(t12248: F, t562: F, t3792: F, t550: F, t1339: F, t836: F, t1336: F, t236: F, t240: F, t10022: F, t248: F, t557: F) -> (F, F, F, F, F, F) {
    let t12249 = t12248 * t562;
    let t12250 = t3792 * t550;
    let t12282 = t1339 * t836;
    let t12283 = t1336 * t12282;
    let t12289 = t12248 * t236;
    let t12290 = t12289 * t240;
    let t12291 = t1336 * t12290;
    let t12328 = t10022 * t557 * t248;
    (t12249, t12250, t12283, t12289, t12291, t12328)
}
