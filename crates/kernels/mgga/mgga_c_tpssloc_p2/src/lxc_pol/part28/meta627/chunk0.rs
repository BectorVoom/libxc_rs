//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1955/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1955<F: Float>(t26959: F, t6495: F, t26070: F, t7032: F, t26073: F, t26076: F, t23998: F, t7435: F, t23967: F, t26090: F, t23993: F, t46104: F, t7025: F) -> (F, F, F, F, F, F, F, F) {
    let t91890 = F::new(32.0) / F::new(9.0) * t6495 * t26959;
    let t91894 = F::new(32.0) / F::new(9.0) * t26070 * t7032;
    let t91896 = F::new(32.0) / F::new(9.0) * t26073 * t7032;
    let t91898 = F::new(32.0) / F::new(9.0) * t26076 * t7032;
    let t91900 = F::new(32.0) / F::new(9.0) * t7435 * t23998;
    let t91904 = F::new(80.0) / F::new(9.0) * t23967 * t26090;
    let t91905 = t7435 * t23993;
    let t91907 = t46104 * t7025;
    (t91890, t91894, t91896, t91898, t91900, t91904, t91905, t91907)
}
