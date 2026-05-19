//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1233/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1233<F: Float>(t108311: F, t2047: F, t2053: F, t20936: F, t21033: F, t21050: F, t218: F, t259: F, t2718: F, t7087: F, t85129: F, t855: F, t98932: F, t98941: F, t98966: F, t98983: F, t98993: F, t98995: F) -> F {
    let t108430 = F::cast_from(0.23029076935875170111e0_f64) * t98932 - t85129 - F::cast_from(0.46058153871750340221e0_f64) * t98941 + t20936 * t2047 * t259 - F::cast_from(0.49348022005446793095e-1_f64) * t98966 + F::cast_from(0.24674011002723396548e-1_f64) * t98983 + t218 * t108311 * t259 + F::new(2.0) * t855 * t2718 * t2053 * t21033 - F::new(6.0) * t7087 * t21050 - F::cast_from(0.69087230807625510332e0_f64) * t98993 - F::cast_from(0.11514538467937585055e0_f64) * t98995;
    t108430
}
