//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1327/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1327<F: Float>(t119878: F, t607: F, t1410: F, t645: F, t641: F, t1433: F, t31: F, t32: F, t26502: F, t3701: F, t26114: F, t8327: F) -> (F, F, F, F, F, F, F) {
    let t119879 = t119878 * t607;
    let t119883 = t1410 * t645;
    let t119891 = t1410 * t641;
    let t119901 = t1433 * t31 * t607;
    let t119931 = t32 * t607;
    let t120016 = t3701 * t26502;
    let t120067 = F::cast_from(2.0_f64) * t26114 * t8327;
    (t119879, t119883, t119891, t119901, t119931, t120016, t120067)
}
