//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1066/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1066<F: Float>(t23076: F, t241: F, t67: F, t2559: F, t2570: F, t782: F, t9558: F, t786: F, t9569: F, t222: F, t39934: F, t2691: F, t812: F, t815: F, t238: F, t244: F, t248: F, t40445: F) -> (F, F, F, F, F, F, F) {
    let t40971 = t241 * t23076 * t67;
    let t41008 = t2559 * t2570;
    let t41011 = t782 * t9558;
    let t41083 = t9569 * t786;
    let t41096 = 455.0 / 243.0 * t39934 * t222;
    let t41115 = t812 * t815 * t2691;
    let t41139 = 13685.0 / 31104.0 * t238 * t40445 * t244 * t248;
    (t40971, t41008, t41011, t41083, t41096, t41115, t41139)
}
