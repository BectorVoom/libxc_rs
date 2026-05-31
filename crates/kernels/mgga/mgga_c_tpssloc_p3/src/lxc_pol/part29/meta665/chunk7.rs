//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2217/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2217<F: Float>(t22633: F, t26421: F, t3856: F, t6976: F, t12267: F, t1336: F, t22873: F, t5287: F, t7745: F, t81069: F, t81073: F, t81075: F, t81076: F, t81083: F, t81099: F, t90903: F, t90907: F, t90910: F, t90913: F, t90917: F, t90921: F, t90925: F, t90929: F) -> F {
    let t90933 = t22633 * t6976 * t26421 * t3856;
    let t90939 = t90903 + F::cast_from(0.3289868133696452873e-1_f64) * t90907 + F::cast_from(0.3289868133696452873e-1_f64) * t90910 - t90913 - F::cast_from(0.9869604401089358619e-1_f64) * t90917 + F::cast_from(0.49348022005446793095e-1_f64) * t90921 - F::cast_from(0.41123351671205660912e-2_f64) * t81069 - t81073 - t81075 + F::cast_from(0.52089578783527170488e-1_f64) * t81076 - t90925 + F::cast_from(0.16449340668482264365e-1_f64) * t81083 + F::cast_from(0.19190897446562641759e-1_f64) * t81099 - F::cast_from(0.82246703342411321825e-2_f64) * t90929 + F::cast_from(0.16449340668482264365e-1_f64) * t90933 - F::cast_from(2.0_f64) * t1336 * t22873 * t5287 - t12267 * t7745;
    t90939
}
