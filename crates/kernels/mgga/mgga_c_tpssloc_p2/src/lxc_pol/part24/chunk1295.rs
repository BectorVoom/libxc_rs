//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1295/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1295<F: Float>(t12030: F, t12033: F, t12444: F, t1390: F, t1983: F, t22670: F, t22905: F, t3758: F, t3889: F, t533: F, t6963: F, t6993: F, t80643: F, t80647: F, t80652: F, t80656: F, t80659: F, t80663: F, t80665: F, t80667: F, t80702: F, t80740: F, t81278: F, t81282: F, t81284: F, t81291: F, t81300: F, t81305: F, t81307: F, t81311: F, t81348: F, t81377: F, t81404: F) -> F {
    let t81410 = t1983 * t533 * (-F::new(3.0) * t12030 * t6993 + F::new(6.0) * t22670 * t3889 - F::new(3.0) * t12033 * t6993 - F::new(3.0) * t3758 * t22905 + F::new(12.0) * t12444 * t6963 + t81404 + t81377 + t81348 - F::cast_from(0.24674011002723396547e-1_f64) * t81311 + F::cast_from(0.49348022005446793095e-1_f64) * t81305 - F::cast_from(0.57572692339687925277e-1_f64) * t81307 - F::cast_from(0.14804406601634037928e0_f64) * t81300 + F::cast_from(0.82246703342411321825e-2_f64) * t81291 + F::cast_from(0.49348022005446793095e-1_f64) * t81284 + t81282 + t81278 + t80740 + t80702 + F::cast_from(0.23029076935875170111e0_f64) * t80665 + F::cast_from(0.11514538467937585055e0_f64) * t80667 - F::cast_from(0.19190897446562641759e0_f64) * t80663 - F::cast_from(0.16449340668482264365e-1_f64) * t80656 + F::cast_from(0.24674011002723396548e-1_f64) * t80659 + F::cast_from(0.9869604401089358619e-1_f64) * t80652 + F::cast_from(0.24674011002723396547e-1_f64) * t80647 - F::cast_from(0.49348022005446793095e-1_f64) * t80643) * t1390;
    t81410
}
