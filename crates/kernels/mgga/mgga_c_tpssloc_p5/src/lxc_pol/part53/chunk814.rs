//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 814/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk814<F: Float>(t22833: F, t5303: F, t1351: F, t16311: F, t3788: F, t6936: F, t16306: F, t550: F, t1339: F, t1887: F, t22839: F) -> (F, F, F, F, F, F) {
    let t26314 = t22833 * t5303;
    let t26318 = t16311 * t1351;
    let t26319 = t3788 * t26318;
    let t26320 = t6936 * t26319;
    let t26322 = t16306 * t550;
    let t26323 = t1339 * t26322;
    let t26324 = t6936 * t26323;
    let t26331 = t22839 * t1887;
    (t26314, t26318, t26320, t26322, t26324, t26331)
}
