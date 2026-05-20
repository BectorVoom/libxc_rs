//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2770/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2770<F: Float>(t16898: F, t9638: F, t13258: F, t16893: F, t16918: F, t4191: F, t46657: F, t4240: F, t120: F, t13076: F, t13171: F, t13251: F, t13326: F, t16662: F, t16839: F, t16896: F, t16901: F, t16976: F, t17009: F, t2643: F, t2645: F, t2679: F, t2684: F, t2707: F, t41448: F, t4178: F, t4180: F, t4181: F, t46549: F, t46551: F, t5624: F, t829: F, t9642: F, t9646: F, t9990: F) -> F {
    let t58461 = t9638 * t16898;
    let t58472 = t13258 * t16893;
    let t58474 = t9638 * t16918;
    let t58480 = t46657 * t4191;
    let t58482 = t46657 * t4240;
    let t58486 = -t16976 * t2707 / F::new(768.0) + F::new(5.0) / F::new(768.0) * t9990 * t5624 + F::new(595.0) / F::new(864.0) * t46549 - F::new(35.0) / F::new(288.0) * t46551 - t9642 * t17009 / F::new(768.0) - t2643 * t4180 * t4181 * t13171 / F::new(1536.0) - F::new(5.0) / F::new(768.0) * t2643 * t9646 * t16896 * t2684 + F::new(5.0) / F::new(384.0) * t4178 * t9646 * t16839 * t41448 + F::new(35.0) / F::new(576.0) * t58461 + t2643 * t2645 * t120 * t16662 * t829 / F::new(384.0) + t2643 * t2645 * t16901 * t2679 / F::new(768.0) - F::new(7.0) / F::new(1152.0) * t58472 - F::new(7.0) / F::new(576.0) * t58474 - t13251 * t13076 / F::new(1536.0) + t9642 * t16918 / F::new(384.0) - F::new(7.0) / F::new(288.0) * t58480 + F::new(7.0) / F::new(1152.0) * t58482 + t13251 * t13326 / F::new(384.0);
    t58486
}
