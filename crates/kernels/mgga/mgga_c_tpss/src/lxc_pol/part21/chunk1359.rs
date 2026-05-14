//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1359/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1359<F: Float>(t4341: F, t5531: F, t626: F, t19434: F, t3499: F, t12664: F, t1688: F, t18540: F, t6243: F, t18547: F, t42962: F, t7029: F, t13554: F, t5532: F, t17907: F, t3493: F) -> (F, F, F, F, F, F, F) {
    let t66033 = 4.0 * t626 * t4341 * t5531;
    let t66035 = 4.0 * t3499 * t19434;
    let t66038 = 2.0 * t626 * t12664 * t1688;
    let t66042 = 6.0 * t6243 * t18540;
    let t66046 = 6.0 * t18547 * t7029 * t42962;
    let t66048 = 4.0 * t13554 * t5532;
    let t66050 = 2.0 * t3493 * t17907;
    (t66033, t66035, t66038, t66042, t66046, t66048, t66050)
}
