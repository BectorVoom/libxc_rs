//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1052/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1052<F: Float>(t22739: F, t22903: F, t1378: F, t22751: F, t6892: F, t6883: F, t6908: F, t2015: F, t3911: F, t3887: F, t3719: F, t6890: F, t6889: F, t6888: F, t22674: F, t6891: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22904 = t22739 + t22903;
    let t22905 = t1378 * t22904;
    let t22907 = t22751 * t6892;
    let t22908 = 0.76763589786250567036e-1 * t22907;
    let t22909 = t6883 * t6908;
    let t22910 = 0.38381794893125283518e-1 * t22909;
    let t22912 = t2015 * t3911;
    let t22913 = t3887 * t22912;
    let t22916 = t6890 * t3719;
    let t22917 = t6889 * t22916;
    let t22918 = t6888 * t22917;
    let t22920 = t22674 * t6891;
    (t22904, t22905, t22908, t22910, t22913, t22916, t22917, t22918, t22920)
}
