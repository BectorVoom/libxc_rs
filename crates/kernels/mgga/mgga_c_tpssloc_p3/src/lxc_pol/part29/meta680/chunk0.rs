//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2286/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2286<F: Float>(t24574: F, t27383: F, t7288: F, t94490: F, t11613: F, t1190: F, t15820: F, t24634: F, t24880: F, t24883: F, t24887: F, t27406: F, t27426: F, t27721: F, t27742: F, t27747: F, t3481: F, t3487: F, t3593: F, t498: F, t5089: F, t7283: F, t7356: F, t8054: F, t8061: F, t86390: F) -> F {
    let t94628 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27383;
    let t94631 = t94490 * t7288;
    let t94637 = F::new(4.0) * t11613 * t8061 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t27426 * t24883 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t27426 * t24887 + F::new(4.0) * t3593 * t27747 + F::cast_from(0.27415567780803773942e-2_f64) * t86390 - F::new(2.0) * t3487 * t27742 + t3481 * t8054 * t498 + F::new(2.0) * t1190 * t27721 * t498 + t94628 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t24634 + F::cast_from(0.48738787165873375896e-2_f64) * t94631 + F::new(4.0) * t15820 * t7356 - F::new(2.0) * t24880 * t5089;
    t94637
}
