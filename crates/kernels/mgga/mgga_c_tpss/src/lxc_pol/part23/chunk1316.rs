//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1316/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1316<F: Float>(t1206: F, t13965: F, t19620: F, t7029: F, t19610: F, t61801: F, t18547: F, t19609: F, t22964: F, t17901: F, t6243: F, t4549: F, t5776: F, t13220: F, t547: F, t5772: F) -> (F, F, F, F, F, F) {
    let t66051 = t13965 * t1206;
    let t66054 = 12.0 * t19620 * t7029 * t66051;
    let t66056 = 6.0 * t61801 * t19610;
    let t66059 = 6.0 * t18547 * t22964 * t19609;
    let t66060 = t6243 * t17901;
    let t66068 = 6.0 * t4549 * t5776;
    let t66073 = 6.0 * t547 * t5772 * t13220;
    (t66054, t66056, t66059, t66060, t66068, t66073)
}
