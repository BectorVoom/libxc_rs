//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1065/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1065<F: Float>(t1878: F, t22683: F, t22844: F, t6604: F, t22779: F, t7712: F, t22759: F, t242: F, t1336: F, t1887: F, t22839: F, t1799: F, t567: F, t1377: F, t22674: F, t7700: F) -> (F, F, F, F, F, F, F, F) {
    let t26284 = t1878 * t22683;
    let t26288 = t22844 * t6604;
    let t26295 = t22779 * t7712;
    let t26308 = t22759 * t242;
    let t26309 = t1336 * t26308;
    let t26331 = t22839 * t1887;
    let t26332 = t567 * t1799;
    let t26337 = t1377 * t1799;
    let t26344 = t22674 * t7700;
    (t26284, t26288, t26295, t26309, t26331, t26332, t26337, t26344)
}
