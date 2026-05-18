//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1151/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1151<F: Float>(t31003: F, t9231: F, t131: F, t31009: F, t9239: F, t2240: F, t6489: F, t79: F, t8306: F, t39063: F, t31016: F, t22642: F, t22643: F, t8458: F) -> (F, F, F, F, F, F, F, F, F) {
    let t113851 = t9231 * t31003;
    let t113861 = t31009 * t131;
    let t113862 = t9239 * t113861;
    let t113869 = t2240 * t6489 * t131;
    let t113874 = t2240 * t113861;
    let t113875 = t8306 * t79;
    let t113880 = t9231 * t31009;
    let t113883 = t39063 * t31003;
    let t113888 = t9239 * t31016;
    let t113934 = F::new(0.16449340668482264365e-1) * t22642 * t22643 * t8458;
    (t113851, t113862, t113869, t113874, t113875, t113880, t113883, t113888, t113934)
}
