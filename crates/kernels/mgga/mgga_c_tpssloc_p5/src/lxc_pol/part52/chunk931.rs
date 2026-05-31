//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 931/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk931<F: Float>(t242: F, t6943: F, t1336: F, t1878: F, t557: F, t556: F, t598: F, t281: F, t6931: F, t1351: F, t22705: F, t236: F, t550: F) -> (F, F, F, F, F) {
    let t22832 = t6943 * t242;
    let t22833 = t1336 * t22832;
    let t22839 = t1878 * t557;
    let t22842 = t556 * t556;
    let t22843 = F::cast_from(1.0_f64) / t22842;
    let t22844 = t598 * t22843;
    let t22852 = t6931 * t281;
    let t22855 = t22705 * t236 * t1351 * t550;
    (t22833, t22839, t22844, t22852, t22855)
}
