//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 362/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk362<F: Float>(t1274: F, t1276: F, t1288: F, t1293: F, t1296: F, t1297: F, t1390: F, t1789: F, t1791: F, t1799: F, t1845: F, t193: F, t533: F, t680: F, t705: F) -> F {
    let t1849 = t1390 * t1845 * t193 * t533 + F::new(3.0) * t1297 * t1799 * t193 - t1274 - t1276 + t1288 - t1293 - t1296 + t1789 + t1791 + t680 + t705;
    t1849
}
