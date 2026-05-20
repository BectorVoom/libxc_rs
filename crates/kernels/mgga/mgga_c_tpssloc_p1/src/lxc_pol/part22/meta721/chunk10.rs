//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2354/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2354<F: Float>(t1509: F, t5631: F, t5611: F, t9975: F, t13380: F, t13397: F, t1510: F, t1523: F, t16673: F, t16811: F, t17030: F, t20876: F, t20986: F, t25115: F, t2617: F, t4166: F, t4281: F, t4282: F, t4286: F, t4291: F, t58181: F, t58262: F, t59331: F, t67739: F, t828: F, t829: F) -> (F, F) {
    let t68217 = t5631 * t1509;
    let t68246 = t9975 * t5611;
    let t68256 = -F::new(18.0) * t13397 * t4282 * t68246 * t828 + F::new(6.0) * t13380 * t20986 * t4281 - F::new(3.0) * t1510 * t4291 * t58262 - F::new(3.0) * t1510 * t4291 * t59331 - F::new(3.0) * t17030 * t25115 * t4291 + F::new(6.0) * t4281 * t4282 * t67739 - F::new(3.0) * t4291 * t68217 * t829 - F::new(3.0) * t1523 * t58181 - F::new(3.0) * t16673 * t4286 + F::new(6.0) * t16811 * t4166 - F::new(3.0) * t20876 * t2617;
    (t68217, t68256)
}
