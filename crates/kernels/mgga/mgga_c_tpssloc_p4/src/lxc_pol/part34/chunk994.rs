//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 994/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk994<F: Float>(t1878: F, t557: F, t556: F, t598: F, t213: F, t281: F, t6931: F, t2003: F, t3862: F, t1887: F, t22715: F, t534: F) -> (F, F, F, F, F, F, F, F) {
    let t22839 = t1878 * t557;
    let t22842 = t556 * t556;
    let t22843 = F::cast_from(1.0_f64) / t22842;
    let t22844 = t598 * t22843;
    let t22845 = t22844 * t213;
    let t22852 = t6931 * t281;
    let t22858 = t2003 * t3862;
    let t22863 = t22715 * t534 * t1887;
    (t22839, t22842, t22843, t22844, t22845, t22852, t22858, t22863)
}
