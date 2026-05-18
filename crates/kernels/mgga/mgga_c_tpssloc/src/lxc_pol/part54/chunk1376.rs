//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1376/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1376<F: Float>(t114770: F, t22986: F, t25192: F, t118833: F, t23270: F, t31332: F, t1888: F, t25045: F, t33447: F, t82159: F, t33371: F, t6547: F) -> (F, F, F, F, F) {
    let t121413 = t22986 * t114770 * t25192;
    let t121419 = t22986 * t23270 * t31332 * t118833;
    let t121426 = t1888 * t114770 * t25045;
    let t121429 = t22986 * t82159 * t33447;
    let t121431 = t6547 * t33371;
    (t121413, t121419, t121426, t121429, t121431)
}
