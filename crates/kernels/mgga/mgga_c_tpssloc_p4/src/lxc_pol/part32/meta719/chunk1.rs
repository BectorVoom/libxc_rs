//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2286/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2286<F: Float>(t3941: F, t4072: F, t7467: F, t28017: F, t3938: F, t12524: F, t28899: F, t75795: F, t7769: F, t26135: F, t5371: F, t16524: F, t26550: F) -> (F, F, F, F, F, F) {
    let t100893 = F::cast_from(54.0_f64) * t3941 * t7467 * t4072;
    let t100897 = F::cast_from(0.135e2_f64) * t3938 * t28017;
    let t100899 = F::cast_from(27.0_f64) * t12524 * t28899;
    let t100902 = F::cast_from(54.0_f64) * t75795 * t7769;
    let t100908 = F::cast_from(27.0_f64) * t5371 * t26135;
    let t100915 = F::cast_from(54.0_f64) * t16524 * t26550;
    (t100893, t100897, t100899, t100902, t100908, t100915)
}
