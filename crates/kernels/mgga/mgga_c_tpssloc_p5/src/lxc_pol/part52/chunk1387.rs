//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1387/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1387<F: Float>(t120665: F, t120672: F, t120675: F, t120677: F, t123205: F, t123206: F, t123211: F, t123213: F, t123215: F, t123217: F, t123220: F, t1266: F, t33686: F, t33756: F, t652: F, t671: F) -> F {
    let t123222 = -F::new(2.0) * t33756 * t652 * t671 - t1266 * t33686 + F::new(2.0) * t120665 - t120672 + t120675 - t120677 - t123205 - F::new(2.0) * t123206 - F::new(2.0) * t123211 - F::new(2.0) * t123213 - F::new(2.0) * t123215 - F::new(2.0) * t123217 + F::new(3.0) * t123220;
    t123222
}
