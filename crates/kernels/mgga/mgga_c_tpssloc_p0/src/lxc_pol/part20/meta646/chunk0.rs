//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2373/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2373<F: Float>(t291: F, t48702: F, t48722: F, t10709: F, t4483: F, t10661: F, t10662: F, t1557: F, t10817: F, t14382: F, t14385: F, t42143: F) -> (F, F, F, F, F) {
    let t48725 = F::new(0.621814e-1) * (t48702 + t48722) * t291;
    let t48727 = F::cast_from(0.35089341735807877242e1_f64) * t4483 * t10709;
    let t48730 = F::new(24.0) * t10661 * t1557 * t10662;
    let t48732 = F::new(6.0) * t10817 * t14382;
    let t48734 = F::cast_from(0.28947563097646563121e3_f64) * t42143 * t14385;
    (t48725, t48727, t48730, t48732, t48734)
}
