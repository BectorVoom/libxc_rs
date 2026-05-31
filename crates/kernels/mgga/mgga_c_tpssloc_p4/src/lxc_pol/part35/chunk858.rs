//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 858/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk858<F: Float>(t10544: F, t2840: F, t287: F, t275: F, t10294: F, t891: F, t2843: F, t290: F, t10629: F, t315: F, t2884: F, t307: F) -> (F, F, F, F, F, F, F, F) {
    let t10636 = F::cast_from(0.55403703703703703703e-1_f64) * t10544;
    let t10660 = F::cast_from(1.0_f64) / t2840 / t287;
    let t10661 = t275 * t10660;
    let t10675 = F::cast_from(0.36514074074074074075e0_f64) * t10294;
    let t10676 = F::cast_from(0.93011851851851851854e0_f64) * t10544;
    let t10701 = F::cast_from(1.0_f64) / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = F::cast_from(1.0_f64) / t2843 / t290;
    let t10756 = t315 * t10629;
    let t10770 = F::cast_from(1.0_f64) / t2884 / t307;
    (t10636, t10661, t10675, t10676, t10702, t10704, t10756, t10770)
}
