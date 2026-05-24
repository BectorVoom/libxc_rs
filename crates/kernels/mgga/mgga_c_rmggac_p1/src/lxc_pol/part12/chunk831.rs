//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 831/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk831<F: Float>(t1587: F, t236: F, t3352: F, t495: F, t7230: F, t1528: F, t3351: F, t498: F, t9210: F, t38539: F, t38541: F, t38545: F, t38550: F, t38552: F, t38554: F, t38556: F, t38560: F, t38563: F, t38566: F, t38570: F, t38572: F, t38574: F, t38576: F, t38578: F) -> F {
    let t38583 = t7230 * t3352 * t236 * t1587 * t495;
    let t38588 = t3351 * t9210 * t236 * t1528 * t498;
    let t38590 = -F::cast_from(0.31923449919973379548e-4_f64) * t38539 + F::cast_from(0.85129199786595678796e-5_f64) * t38541 + F::cast_from(0.85129199786595678796e-5_f64) * t38545 + F::cast_from(0.31923449919973379548e-4_f64) * t38550 + F::cast_from(0.30487649791575028314e-3_f64) * t38552 + F::cast_from(0.30487649791575028314e-3_f64) * t38554 - F::cast_from(0.35220688045884876043e-2_f64) * t38556 - t38560 - t38563 + F::cast_from(0.20455996240684006296e0_f64) * t38566 - F::cast_from(0.72732431077987577942e-1_f64) * t38570 + F::cast_from(0.51077519871957407276e-4_f64) * t38572 - F::cast_from(0.76616279807936110914e-4_f64) * t38574 - F::cast_from(0.25538759935978703638e-4_f64) * t38576 + F::cast_from(0.25538759935978703638e-4_f64) * t38578 - F::cast_from(0.31923449919973379548e-4_f64) * t38583 + F::cast_from(0.17025839957319135759e-4_f64) * t38588;
    t38590
}
