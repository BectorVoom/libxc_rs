//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2139/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2139<F: Float>(t22674: F, t28205: F, t6897: F, t12021: F, t1375: F, t16460: F, t20026: F, t26477: F, t5354: F, t6439: F, t6958: F, t6992: F, t7729: F, t80663: F, t80671: F, t90460: F, t90469: F, t90471: F, t90473: F, t90498: F, t90501: F, t96848: F, t96851: F, t96854: F, t96857: F, t96866: F, t96868: F, t96873: F) -> F {
    let t96878 = t6897 * t22674 * t28205;
    let t96885 = -F::cast_from(0.24674011002723396548e-1_f64) * t96848 + F::cast_from(0.16449340668482264365e-1_f64) * t96851 + t90460 + F::cast_from(0.9869604401089358619e-1_f64) * t96854 + t90469 + t90471 - t90473 - F::cast_from(0.82246703342411321825e-2_f64) * t96857 + F::cast_from(4.0_f64) * t16460 * t7729 - F::cast_from(6.0_f64) * t1375 * t12021 * t6992 * t6439 - F::cast_from(0.16449340668482264365e-1_f64) * t96866 + F::cast_from(0.19190897446562641759e-1_f64) * t96868 + F::cast_from(0.16449340668482264365e-1_f64) * t96873 + F::cast_from(2.0_f64) * t6958 * t20026 + F::cast_from(0.41123351671205660912e-2_f64) * t96878 - F::cast_from(2.0_f64) * t26477 * t5354 - F::cast_from(0.63969658155208805863e-1_f64) * t80663 - F::cast_from(0.52089578783527170488e-1_f64) * t80671 - F::cast_from(0.23029076935875170111e0_f64) * t90498 - t90501;
    t96885
}
