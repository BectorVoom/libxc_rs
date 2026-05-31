//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2335/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2335<F: Float>(t75795: F, t7769: F, t26135: F, t5371: F, t112: F, t28868: F, t16524: F, t26550: F, t55353: F, t16521: F, t7467: F, t1873: F, t19534: F, t3941: F) -> (F, F, F, F, F, F, F) {
    let t100902 = F::cast_from(54.0_f64) * t75795 * t7769;
    let t100908 = F::cast_from(27.0_f64) * t5371 * t26135;
    let t100911 = t28868 * t112;
    let t100915 = F::cast_from(54.0_f64) * t16524 * t26550;
    let t100917 = F::cast_from(54.0_f64) * t55353 * t7769;
    let t100921 = F::cast_from(27.0_f64) * t16521 * t7467;
    let t100924 = F::cast_from(27.0_f64) * t3941 * t1873 * t19534;
    (t100902, t100908, t100911, t100915, t100917, t100921, t100924)
}
