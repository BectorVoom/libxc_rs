//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 973/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk973<F: Float>(t32503: F, t462: F, t1238: F, t1252: F, t2121: F, t32422: F, t32452: F, t32480: F, t32482: F, t32489: F, t32493: F, t32498: F, t32499: F, t3487: F, t3593: F, t498: F, t7283: F, t7351: F, t7356: F, t8888: F, t8898: F) -> (F, F) {
    let t32504 = t462 * t32503;
    let t32507 = 4.0 * t7351 * t7356 + t32422 * t498 + t32452 * t498 - t1238 * t32480 - t32482 * t1252 - t3593 * t8898 - t3487 * t8898 + 2.0 * t3593 * t8888 + 2.0 * t1238 * t32489 + 4.0 * t1238 * t32493 + t32498 - 0.16449340668482264365e-1 * t7283 * t32499 + 0.16449340668482264365e-1 * t2121 * t32504;
    (t32504, t32507)
}
