//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1147/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1147<F: Float>(t120114: F, t120171: F, t120176: F, t120658: F, t120663: F, t120672: F, t120677: F, t120683: F, t123195: F, t123199: F, t123205: F, t123206: F, t123211: F, t123213: F, t123215: F, t123217: F, t123220: F, t125903: F, t510: F) -> (F,) {
    let t125951 = -t125903 * t510 - t120114 + t120171 - t120176 + t120658 + t120663 - t120672 - t120677 - t120683 + 4.0 * t123195 + 12.0 * t123199 - 2.0 * t123205 - 4.0 * t123206 - 4.0 * t123211 - 4.0 * t123213 - 4.0 * t123215 - 4.0 * t123217 + 6.0 * t123220;
    (t125951,)
}
