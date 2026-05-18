//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 223/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk223<F: Float>(t514: F, t517: F, t215: F, t535: F, t782: F, t154: F, t547: F, t205: F, t792: F, t795: F, t541: F, t801: F) -> (F, F, F, F, F, F, F) {
    let t1298 = F::new(1.0) / t514;
    let t1302 = F::new(1.0) / t517;
    let t1313 = F::new(0.19444444444444444444e-2) * t782 * t535 * t215;
    let t1314 = t154 * t547;
    let t1315 = t205 * t1314;
    let t1322 = F::new(0.41666666666666666666e-3) * t792 * t535 * t795;
    let t1327 = F::new(7.0) / F::new(288.0) * t801 * t541;
    (t1298, t1302, t1313, t1314, t1315, t1322, t1327)
}
