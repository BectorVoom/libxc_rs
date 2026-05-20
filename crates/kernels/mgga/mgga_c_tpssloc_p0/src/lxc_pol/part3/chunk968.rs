//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 968/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk968<F: Float>(t1338: F, t3879: F, t3773: F, t68: F, t1339: F, t836: F, t1336: F, t3809: F, t12248: F, t236: F, t3777: F, t3798: F) -> (F, F, F, F, F, F) {
    let t12259 = t1338 * t3879;
    let t12267 = t3773 * t68;
    let t12282 = t1339 * t836;
    let t12283 = t1336 * t12282;
    let t12284 = t12283 * t3809;
    let t12289 = t12248 * t236;
    let t12300 = t3777 * t3798;
    (t12259, t12267, t12283, t12284, t12289, t12300)
}
