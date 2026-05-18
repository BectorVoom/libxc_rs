//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1230/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1230<F: Float>(t12020: F, t2015: F, t1878: F, t22683: F, t22844: F, t6604: F, t22759: F, t242: F, t1336: F, t1887: F, t22839: F, t552: F) -> (F, F, F, F, F, F) {
    let t26225 = t12020 * t2015;
    let t26284 = t1878 * t22683;
    let t26288 = t22844 * t6604;
    let t26308 = t22759 * t242;
    let t26309 = t1336 * t26308;
    let t26331 = t22839 * t1887;
    let t26446 = t6604 * t552;
    (t26225, t26284, t26288, t26309, t26331, t26446)
}
