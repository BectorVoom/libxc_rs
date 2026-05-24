//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 695/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk695<F: Float>(t675: F, t9938: F, t2402: F, t558: F, t884: F, t1707: F, t645: F, t3928: F, t2060: F, t6522: F, t1550: F, t2024: F, t6557: F) -> (F, F, F, F, F, F, F, F) {
    let t9939 = t675 * t9938;
    let t9940 = F::cast_from(0.51077519871957407276e-4_f64) * t9939;
    let t9944 = t2402 * t558;
    let t9945 = t884 * t9944;
    let t9946 = F::cast_from(0.11974241701863808564e0_f64) * t9945;
    let t9948 = t645 * t1707;
    let t9949 = t3928 * t9948;
    let t9950 = F::cast_from(0.17961362552795712846e0_f64) * t9949;
    let t9951 = t2060 * t6522;
    let t9952 = t1550 * t9951;
    let t9953 = F::cast_from(0.5987120850931904282e-1_f64) * t9952;
    let t9954 = t2024 * t6557;
    (t9940, t9944, t9946, t9948, t9950, t9951, t9953, t9954)
}
