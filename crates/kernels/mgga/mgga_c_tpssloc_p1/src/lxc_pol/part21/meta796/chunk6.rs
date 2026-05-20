//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2764/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2764<F: Float>(t1519: F, t4233: F, t2631: F, t40933: F, t13263: F, t13390: F, t13397: F, t13433: F, t16758: F, t16815: F, t16828: F, t17023: F, t17030: F, t2613: F, t2633: F, t2679: F, t2684: F, t4234: F, t4281: F, t4291: F, t47386: F, t5655: F, t58166: F, t808: F, t812: F, t829: F, t9632: F) -> (F, F, F) {
    let t58226 = t1519 * t4233;
    let t58246 = t40933 * t2631;
    let t58261 = -F::new(36.0) * t13263 * t13397 * t16815 - F::new(4.0) * t13433 * t4234 * t812 + F::new(12.0) * t16758 * t2633 * t4281 - F::new(2.0) * t16758 * t2679 * t4291 - t16815 * t2684 * t4291 + F::new(6.0) * t16815 * t4281 * t9632 + F::new(24.0) * t16815 * t47386 * t58246 - t17030 * t2679 * t4291 - F::new(2.0) * t4291 * t58166 * t829 - F::new(4.0) * t4291 * t58226 * t829 - F::new(2.0) * t13390 * t16828 + F::new(2.0) * t17023 * t808 + t2613 * t5655;
    (t58226, t58246, t58261)
}
