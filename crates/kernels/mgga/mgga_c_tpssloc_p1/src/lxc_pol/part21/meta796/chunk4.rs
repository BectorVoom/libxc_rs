//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2762/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2762<F: Float>(t17083: F, t225: F, t5584: F, t852: F, t16805: F, t68: F, t10076: F, t13171: F, t13263: F, t13381: F, t13388: F, t13390: F, t13397: F, t13456: F, t16758: F, t16816: F, t16830: F, t17030: F, t17046: F, t2633: F, t4162: F, t4281: F, t4282: F, t4290: F, t4291: F, t4292: F, t4295: F, t5612: F, t812: F, t861: F) -> (F, F, F, F) {
    let t58143 = t17083 * t225;
    let t58166 = t852 * t5584;
    let t58181 = t16805 * t68;
    let t58194 = -t10076 * t5612 * t812 - F::cast_from(2.0_f64) * t13171 * t4282 * t4291 - F::cast_from(2.0_f64) * t13171 * t4295 * t812 - F::cast_from(12.0_f64) * t13263 * t13397 * t16758 - F::cast_from(6.0_f64) * t13263 * t13397 * t17030 - F::cast_from(12.0_f64) * t13397 * t16816 * t58166 + F::cast_from(6.0_f64) * t17030 * t2633 * t4281 - F::cast_from(4.0_f64) * t4162 * t4290 * t4292 - F::cast_from(4.0_f64) * t13381 * t16830 - F::cast_from(2.0_f64) * t13388 * t16830 - F::cast_from(2.0_f64) * t13390 * t17046 - F::cast_from(4.0_f64) * t13456 * t16830 - F::cast_from(2.0_f64) * t58181 * t861;
    (t58143, t58166, t58181, t58194)
}
