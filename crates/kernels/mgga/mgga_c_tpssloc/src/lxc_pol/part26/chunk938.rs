//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 938/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk938<F: Float>(t28: F, t265: F, t504: F, t10150: F, t11476: F, t11955: F, t1081: F, t11122: F, t1260: F, t2250: F, t2756: F, t3231: F, t3644: F, t506: F, t52: F, t607: F, t873: F, t9258: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t11957 = piecewise3(t505, t11476 + t11955, t10150);
    let t11967 = piecewise3(t401, t10150 * t28 / 2.0 + 3.0 / 2.0 * t2756 * t1081 + 3.0 / 2.0 * t873 * t3231 + t265 * t11122 / 2.0, t11957 * t52 / 2.0 - 3.0 / 2.0 * t3644 * t607 - 3.0 / 2.0 * t1260 * t2250 - t506 * t9258 / 2.0);
    (t11967,)
}
