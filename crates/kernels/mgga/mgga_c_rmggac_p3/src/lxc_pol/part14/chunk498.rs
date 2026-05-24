//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 498/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk498<F: Float>(t1415: F, t381: F, t4352: F, t183: F, t5415: F, t155: F, t1042: F, t1372: F, t1138: F, t1435: F, t1392: F, t446: F) -> (F, F, F, F, F, F) {
    let t5425 = t381 * t1415;
    let t5426 = F::new(8.0) * t5425;
    let t5427 = F::cast_from(0.4883052614935078681e-3_f64) * t4352;
    let t5428 = t5415 * t183;
    let t5429 = t155 * t5428;
    let t5432 = t1372 * t1042;
    let t5433 = F::cast_from(0.17315859105681463759e2_f64) * t5432;
    let t5434 = t1435 * t1138;
    let t5435 = F::cast_from(0.24415263074675393405e-3_f64) * t5434;
    let t5436 = t1392 * t446;
    (t5426, t5427, t5429, t5433, t5435, t5436)
}
