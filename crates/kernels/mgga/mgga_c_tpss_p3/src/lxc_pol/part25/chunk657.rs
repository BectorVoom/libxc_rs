//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 657/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk657<F: Float>(t1519: F, t2857: F, t1042: F, t2862: F, t1509: F, t2868: F, t1027: F, t2836: F, t2872: F, t4044: F, t4049: F, t4054: F, t4058: F) -> (F, F, F, F, F, F) {
    let t4067 = F::new(1.0) * t2857 * t1519;
    let t4068 = t1519 * t1042;
    let t4070 = F::new(2.0) * t2862 * t4068;
    let t4071 = t2868 * t1509;
    let t4072 = t4071 * t1027;
    let t4079 = t2872 - t2836 / F::new(9.0) - t4044 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t4049 + F::new(2.0) / F::new(3.0) * t4054 + t4058 / F::new(3.0);
    (t4067, t4068, t4070, t4071, t4072, t4079)
}
