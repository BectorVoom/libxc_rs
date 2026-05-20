//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2328/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2328<F: Float>(t24574: F, t27574: F, t24844: F, t7999: F, t1244: F, t1246: F, t15015: F, t15027: F, t1729: F, t24792: F, t24863: F, t27470: F, t27724: F, t3471: F, t3493: F, t3624: F, t470: F, t493: F, t5079: F, t7283: F, t7373: F, t7375: F, t7376: F, t8054: F, t8077: F, t86020: F, t95707: F) -> F {
    let t95714 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27574;
    let t95722 = F::cast_from(0.14621636149762012769e-1_f64) * t7999 * t24844;
    let t95723 = t1729 * t24792 - F::new(2.0) * t3624 * t27724 * t5079 - F::new(2.0) * t3624 * t27470 * t5079 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t7375 * t15015 * t7376 + t470 * t493 * t95707 + t1244 * t8054 * t3493 * t1246 - t95714 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t3471 * t8077 - F::cast_from(0.54831135561607547884e-2_f64) * t86020 + F::new(2.0) * t15027 * t24863 - t95722;
    t95723
}
