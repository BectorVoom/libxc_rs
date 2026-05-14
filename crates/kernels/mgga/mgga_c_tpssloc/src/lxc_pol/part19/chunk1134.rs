//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1134/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1134<F: Float>(t13012: F, t9566: F, t207: F, t215: F, t39933: F, t40344: F, t795: F, t116: F, t786: F, t9534: F, t133: F, t6600: F, t776: F, t13005: F, t213: F, t221: F, t2379: F, t2553: F, t41187: F, t41190: F, t41192: F, t41194: F, t41197: F, t41200: F, t41203: F, t4127: F, t9516: F) -> (F,) {
    let t41205 = t13012 * t9566;
    let t41209 = 0.14979423868312757201e0 * t39933 * t207 * t215;
    let t41212 = 0.11265432098765432099e0 * t40344 * t207 * t795;
    let t41214 = t9534 * t786 * t116;
    let t41217 = t41214 * t133 * t6600 * t776;
    let t41229 = -0.77777777777777777775e-1 * t41187 + 0.13148148148148148148e0 * t41190 - 0.31666666666666666666e-1 * t41192 + 0.23333333333333333332e0 * t41194 + 0.94999999999999999997e-1 * t41197 - t41200 - 0.29999999999999999998e-1 * t41203 - 0.13999999999999999999e0 * t41205 + t41209 + t41212 + 0.11111111111111111111e-2 * t41217 + 0.19999999999999999999e-1 * t4127 * t221 * t213 * t9516 * t776 - 0.11999999999999999999e0 * t13005 * t221 * t213 * t2379 * t2553;
    (t41229,)
}
