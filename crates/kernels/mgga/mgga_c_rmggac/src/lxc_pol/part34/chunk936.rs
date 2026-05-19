//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 936/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk936<F: Float>(t73814: F, t73819: F, t73822: F, t73837: F, t73845: F, t1971: F, t3351: F, t4617: F, t9552: F, t15450: F, t7244: F, t495: F, t7230: F, t875: F, t9551: F) -> (F, F, F, F, F, F, F, F) {
    let t76688 = F::cast_from(0.16351352353374609375e-5_f64) * t73814;
    let t76689 = F::cast_from(0.39726959900411316773e-4_f64) * t73819;
    let t76690 = F::cast_from(0.2553875993597870364e-4_f64) * t73822;
    let t76693 = F::cast_from(0.2553875993597870364e-4_f64) * t73837;
    let t76696 = F::cast_from(0.23268647941669485538e-4_f64) * t73845;
    let t76700 = t3351 * t1971 * t4617 * t9552;
    let t76701 = F::cast_from(0.25538759935978703639e-4_f64) * t76700;
    let t76702 = t7244 * t15450;
    let t76703 = F::cast_from(0.99317399751028291929e-5_f64) * t76702;
    let t76707 = t7230 * t1971 * t875 * t9551 * t495;
    (t76688, t76689, t76690, t76693, t76696, t76701, t76703, t76707)
}
