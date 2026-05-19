//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1349/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1349<F: Float>(t11607: F, t1186: F, t11925: F, t1238: F, t2154: F, t24633: F, t24638: F, t24877: F, t24883: F, t27799: F, t3471: F, t3477: F, t3593: F, t45350: F, t7283: F, t7300: F, t7302: F, t7392: F, t85674: F, t85683: F, t85688: F, t85701: F, t85707: F, t85711: F) -> F {
    let t85713 = -F::cast_from(0.49348022005446793095e-1_f64) * t7283 * t7300 * t85674 * t11607 + F::new(24.0) * t1238 * t45350 * t2154 * t11607 - F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t85683 * t27799 - F::cast_from(0.49348022005446793095e-1_f64) * t7283 * t1186 * t85688 + F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t3471 * t24638 + F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t3477 * t24638 - F::cast_from(0.82246703342411321826e-2_f64) * t7283 * t24633 * t24883 + F::cast_from(0.54831135561607547884e-2_f64) * t85701 + F::new(6.0) * t3593 * t24877 - F::new(3.0) * t11925 * t7392 - F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t85707 * t7302 - F::cast_from(0.82246703342411321826e-2_f64) * t85711;
    t85713
}
