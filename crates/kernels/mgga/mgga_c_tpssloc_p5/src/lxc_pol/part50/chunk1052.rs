//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1052/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1052<F: Float>(t28: F, t265: F, t504: F, t30952: F, t30982: F, t52: F, t607: F, t8435: F, t30958: F, t649: F, t8319: F, t510: F, t1266: F, t8320: F, t8301: F, t9231: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t30983 = piecewise3::<F>(t505, F::new(0.0), t30952);
    let t30988 = piecewise3::<F>(t401, t30982, t30983 * t52 / F::new(2.0) - t8435 * t607 / F::new(2.0));
    let t30989 = t30958 + t30988;
    let t30991 = t649 * t8319;
    let t30993 = F::new(2.0) * t30991 * t510;
    let t30995 = F::new(2.0) * t8320 * t1266;
    let t31000 = t9231 * t8301;
    (t30983, t30989, t30991, t30993, t30995, t31000)
}
