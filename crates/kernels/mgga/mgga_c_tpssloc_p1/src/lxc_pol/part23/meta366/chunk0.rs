//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1166/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1166<F: Float>(t43819: F, t3262: F, t3311: F, t409: F, t43776: F, t3314: F, t3374: F, t3399: F, t440: F, t3355: F, t427: F, t3358: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t43895 = F::cast_from(0.31310740740740740741e1_f64) * t43819;
    let t43942 = F::cast_from(0.96141975308641975307e-1_f64) * t43819;
    let t43969 = t409 / t3311 / t3262;
    let t44027 = F::cast_from(0.13388493827160493828e1_f64) * t43776;
    let t44053 = F::cast_from(0.31003950617283950618e1_f64) * t43819;
    let t44073 = t3311 * t3311;
    let t44075 = t409 / t44073;
    let t44076 = t3314 * t3314;
    let t44077 = F::cast_from(1.0_f64) / t44076;
    let t44154 = F::cast_from(1.0_f64) / t3399 / t3374;
    let t44155 = t440 * t44154;
    let t44175 = t3355 * t3355;
    let t44177 = t427 / t44175;
    let t44178 = t3358 * t3358;
    (t43895, t43942, t43969, t44027, t44053, t44075, t44077, t44154, t44155, t44177, t44178)
}
