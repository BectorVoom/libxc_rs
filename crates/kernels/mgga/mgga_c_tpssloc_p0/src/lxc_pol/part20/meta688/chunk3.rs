//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2608/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2608<F: Float>(t52973: F, t11801: F, t5024: F, t11820: F, t5019: F, t11729: F, t11739: F, t1227: F, t15527: F, t15541: F, t15545: F, t15656: F, t3490: F, t3536: F, t44836: F, t45037: F, t4582: F, t45997: F, t46006: F, t4977: F, t4987: F) -> F {
    let t52974 = t52973 / F::new(4608.0);
    let t52975 = t5024 * t11801;
    let t52987 = t5019 * t11820;
    let t52988 = t52987 / F::new(864.0);
    let t52989 = -t44836 * t4582 * t4977 * t11739 / F::new(3072.0) + F::new(5.0) / F::new(2304.0) * t3490 * t15541 + F::new(5.0) / F::new(4608.0) * t3490 * t15545 + F::new(5.0) / F::new(768.0) * t3490 * t15656 + F::new(7.0) / F::new(1536.0) * t45037 * t4582 * t4977 * t11729 - t52974 + t52975 / F::new(216.0) + t3536 * t15527 / F::new(1024.0) + F::new(5.0) / F::new(4608.0) * t1227 * t4582 * t4987 * t46006 + F::new(5.0) / F::new(4608.0) * t1227 * t4582 * t4987 * t45997 + t52988;
    t52989
}
