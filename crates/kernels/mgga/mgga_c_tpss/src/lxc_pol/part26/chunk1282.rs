//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1282/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1282<F: Float>(t13554: F, t6106: F, t19327: F, t3493: F, t19597: F, t19434: F, t2056: F, t21191: F, t3499: F, t1163: F, t21190: F, t626: F, t6103: F, t19620: F, t51631: F, t7029: F) -> (F, F, F, F, F, F, F, F, F) {
    let t68919 = 4.0 * t13554 * t6106;
    let t68921 = 4.0 * t3493 * t19327;
    let t68923 = 4.0 * t3493 * t19597;
    let t68927 = 4.0 * t3493 * t19434;
    let t68929 = 2.0 * t2056 * t21191;
    let t68931 = 2.0 * t3499 * t21191;
    let t68934 = 2.0 * t626 * t1163 * t21190;
    let t68936 = 4.0 * t6103 * t19434;
    let t68941 = 6.0 * t19620 * t7029 * t51631;
    (t68919, t68921, t68923, t68927, t68929, t68931, t68934, t68936, t68941)
}
