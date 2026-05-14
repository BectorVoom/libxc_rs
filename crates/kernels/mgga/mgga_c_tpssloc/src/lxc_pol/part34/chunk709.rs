//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 709/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk709<F: Float>(t12248: F, t562: F, t3792: F, t550: F, t1339: F, t836: F, t1336: F, t236: F, t240: F, t10022: F, t248: F, t557: F, t555: F, t10027: F, t541: F, t1361: F, t2690: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12249 = t12248 * t562;
    let t12250 = t3792 * t550;
    let t12282 = t1339 * t836;
    let t12283 = t1336 * t12282;
    let t12289 = t12248 * t236;
    let t12290 = t12289 * t240;
    let t12291 = t1336 * t12290;
    let t12328 = t10022 * t557 * t248;
    let t12330 = 595.0 / 10368.0 * t555 * t12328;
    let t12335 = 455.0 / 1296.0 * t10027 * t541;
    let t12344 = t1361 * t2690;
    (t12249, t12250, t12283, t12289, t12291, t12328, t12330, t12335, t12344)
}
