//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1360/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1360<F: Float>(t1206: F, t13965: F, t19620: F, t7029: F, t19610: F, t61801: F, t18547: F, t19609: F, t22964: F, t17901: F, t6243: F, t10456: F, t10464: F, t1273: F, t1663: F, t18389: F, t18430: F, t19448: F, t19457: F, t19667: F, t2056: F, t3493: F, t3538: F, t5514: F, t6117: F, t66033: F, t66035: F, t66038: F, t66042: F, t66046: F, t66048: F, t66050: F, t7798: F) -> (F,) {
    let t66051 = t13965 * t1206;
    let t66054 = 12.0 * t19620 * t7029 * t66051;
    let t66056 = 6.0 * t61801 * t19610;
    let t66059 = 6.0 * t18547 * t22964 * t19609;
    let t66060 = t6243 * t17901;
    let t66061 = -4.0 * t10456 * t6117 - 2.0 * t10464 * t5514 + 2.0 * t1273 * t19667 + t1663 * t18430 - 2.0 * t18389 * t3493 - 4.0 * t19448 * t2056 - 4.0 * t19457 * t3538 - 2.0 * t6117 * t7798 - t66033 - t66035 - t66038 + t66042 - t66046 - t66048 - t66050 - t66054 - t66056 - t66059 - t66060;
    (t66061,)
}
