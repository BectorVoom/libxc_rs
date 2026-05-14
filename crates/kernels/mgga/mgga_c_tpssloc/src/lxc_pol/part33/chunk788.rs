//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 788/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk788<F: Float>(t273: F, t10544: F, t2928: F, t941: F, t2931: F, t323: F, t2840: F, t287: F, t275: F, t10294: F, t891: F, t2843: F, t290: F, t315: F, t2884: F, t307: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10599 = 1.0/pow_3_2(t273);
    let t10608 = 0.28842592592592592592e-1 * t10544;
    let t10629 = 1.0 / t2928 / t941;
    let t10632 = 1.0 / t2931 / t323;
    let t10636 = 0.55403703703703703703e-1 * t10544;
    let t10660 = 1.0 / t2840 / t287;
    let t10661 = t275 * t10660;
    let t10675 = 0.36514074074074074075e0 * t10294;
    let t10676 = 0.93011851851851851854e0 * t10544;
    let t10701 = 1.0 / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = 1.0 / t2843 / t290;
    let t10756 = t315 * t10629;
    let t10770 = 1.0 / t2884 / t307;
    (t10599, t10608, t10629, t10632, t10636, t10661, t10675, t10676, t10702, t10704, t10756, t10770)
}
