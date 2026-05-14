//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1302/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1302<F: Float>(t6024: F, t63755: F, t21810: F, t4740: F, t21813: F, t51120: F, t1164: F, t6088: F, t64537: F, t19270: F, t193: F, t336: F, t3640: F, t4700: F, t6270: F, t78310: F, t78312: F, t78314: F, t78318: F, t78320: F, t78321: F, t78327: F, t78329: F) -> (F, F, F, F, F) {
    let t78331 = 0.96491876992155210402e2 * t63755 * t6024;
    let t78333 = 4.0 * t4740 * t21810;
    let t78335 = 0.2069040516770936012e4 * t51120 * t21813;
    let t78338 = 0.62337092780453269531e3 * t1164 * t64537 * t6088;
    let t78342 = -3.0 * t193 * t336 * t3640 * t78321 + 12.0 * t19270 * t4700 * t6270 + t78310 - t78312 - t78314 - t78318 - t78320 + t78327 + t78329 + t78331 + t78333 + t78335 + t78338;
    (t78331, t78333, t78335, t78338, t78342)
}
