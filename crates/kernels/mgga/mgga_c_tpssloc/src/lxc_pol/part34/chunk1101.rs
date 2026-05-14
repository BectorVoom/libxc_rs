//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1101/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1101<F: Float>(t100966: F, t100972: F, t103091: F, t103092: F, t103098: F, t103099: F, t108858: F, t108871: F, t1398: F, t1852: F, t1858: F, t2099: F, t2105: F, t22431: F, t22453: F, t29396: F, t29430: F, t3: F, t580: F, t6471: F, t6483: F, t7946: F, t7961: F) -> (F,) {
    let tv4rho3sigma10 = t108858 * t3 * t580 + t108871 * t1398 + 3.0 * t1852 * t29430 + 3.0 * t1858 * t29396 + t2099 * t22453 + t2105 * t22431 + 3.0 * t6471 * t7961 + 3.0 * t6483 * t7946 + 3.0 * t100966 + 6.0 * t100972 + 3.0 * t103091 + 6.0 * t103092 + 3.0 * t103098 + 3.0 * t103099;
    (tv4rho3sigma10,)
}
