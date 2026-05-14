//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 976/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk976<F: Float>(t1291: F, t1307: F, t1314: F, t13365: F, t13407: F, t13443: F, t3433: F, t3436: F, t3441: F, t3463: F, t3483: F, t4609: F, t4623: F, t583: F, t603: F, t616: F, t71: F, t85: F) -> (F,) {
    let t13446 = -t3433 * t1314 / 6.0 - t3436 * t1314 / 6.0 - t1291 * t3483 / 6.0 - t13365 * t85 / 12.0 + t13407 * t85 / 24.0 + t4609 * t616 / 24.0 - t3441 * t1314 / 6.0 + t3463 * t1314 / 12.0 + t1307 * t3483 / 12.0 - t583 * t4623 / 12.0 + t603 * t4623 / 24.0 + t71 * t13443 / 24.0;
    (t13446,)
}
