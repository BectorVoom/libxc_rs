//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2578/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2578<F: Float>(t18915: F, t4879: F, t21938: F, t3400: F, t1164: F, t4883: F, t300: F, t71310: F, t1155: F, t1695: F, t51810: F, t6084: F) -> (F, F, F, F) {
    let t72061 = F::cast_from(0.17544670867903938621e1_f64) * t18915 * t4879;
    let t72062 = t3400 * t21938;
    let t72065 = F::cast_from(0.17315859105681463759e2_f64) * t1164 * t72062 * t4883;
    let t72067 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t71310;
    let t72071 = F::cast_from(0.10526802520742363173e2_f64) * t51810 * t1695 * t6084 * t1155;
    (t72061, t72065, t72067, t72071)
}
