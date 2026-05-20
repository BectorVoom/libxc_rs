//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1556/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1556<F: Float>(t761: F, t9905: F, t2250: F, t751: F, t707: F, t2447: F, t706: F, t2509: F, t746: F, t9490: F) -> (F, F, F, F, F) {
    let t9907 = F::cast_from(0.35089341735807877242e1_f64) * t761 * t9905;
    let t9909 = t751 * t2250;
    let t9910 = t707 * t9909;
    let t9912 = t706 * t2447;
    let t9919 = t2509 * t9490 * t746;
    (t9907, t9909, t9910, t9912, t9919)
}
