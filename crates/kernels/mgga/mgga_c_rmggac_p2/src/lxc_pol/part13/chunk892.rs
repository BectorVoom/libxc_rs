//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 892/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk892<F: Float>(t16043: F, t9096: F, t1971: F, t2144: F, t27044: F, t3351: F, t27136: F, t9138: F, t27120: F, t875: F, t27075: F, t1657: F, t2039: F, t270: F, t638: F) -> (F, F, F, F, F, F, F) {
    let t39760 = t16043 * t9096;
    let t39764 = t3351 * t1971 * t2144 * t27044;
    let t39771 = t3351 * t1971 * t2144 * t27136;
    let t39773 = t16043 * t9138;
    let t39777 = t3351 * t1971 * t875 * t27120;
    let t39781 = t3351 * t1971 * t875 * t27075;
    let t39785 = t638 * t2039 * t1657 * t270;
    (t39760, t39764, t39771, t39773, t39777, t39781, t39785)
}
