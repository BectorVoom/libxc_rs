//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 667/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk667<F: Float>(t265: F, t394: F, t202: F, t6665: F, t1877: F, t1915: F, t193: F, t2522: F, t6670: F, t776: F, t868: F, t870: F, t1068: F, t1070: F, t336: F, t4700: F, t6818: F, t6822: F) -> (F, F) {
    let t395 = t265 < t394;
    let t6829 = t202 * t6665;
    let t6834 = -t1877 * t6670 * t868 + 3.0 * t1915 * t2522 * t776 + t193 * t6829 * t870;
    let t6835 = piecewise3(t395, t1070 * t193 * t336 * t6818 - t1068 * t4700 * t6822, t6834);
    (t6834, t6835)
}
