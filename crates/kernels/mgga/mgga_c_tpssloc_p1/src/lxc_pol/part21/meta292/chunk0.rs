//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1606/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1606<F: Float>(t10294: F, t10544: F, t2840: F, t891: F, t275: F) -> (F, F, F, F) {
    let t10675 = F::cast_from(0.36514074074074074075e0_f64) * t10294;
    let t10676 = F::cast_from(0.93011851851851851854e0_f64) * t10544;
    let t10701 = F::new(1.0) / t2840 / t891;
    let t10702 = t275 * t10701;
    (t10675, t10676, t10701, t10702)
}
