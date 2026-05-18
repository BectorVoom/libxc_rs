//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 758/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk758<F: Float>(t2862: F, t5082: F, t1509: F, t2868: F, t2872: F, t4044: F, t5066: F, t5070: F, t5074: F, t1025: F, t2885: F, t1032: F) -> (F, F, F, F, F, F, F) {
    let t5084 = F::new(2.0) * t2862 * t5082;
    let t5085 = t1509 * t1509;
    let t5086 = t2868 * t5085;
    let t5092 = t2872 - F::new(2.0) / F::new(9.0) * t4044 - F::new(2.0) / F::new(9.0) * t5066 + F::new(2.0) / F::new(3.0) * t5070 + t5074 / F::new(3.0);
    let t5093 = t1025 * t5092;
    let t5099 = t2885 * t5085;
    let t5101 = t1032 * t5092;
    (t5084, t5085, t5086, t5092, t5093, t5099, t5101)
}
