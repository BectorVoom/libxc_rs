//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1464/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1464<F: Float>(t78791: F, t78792: F, t78794: F, t79006: F, t6224: F, t11721: F, t1213: F, t1214: F, t15503: F, t19083: F, t22246: F, t22271: F, t22309: F, t248: F, t45030: F, t475: F, t488: F, t5002: F, t53336: F, t6164: F, t6169: F, t6211: F, t65628: F, t65632: F, t65647: F, t65664: F, t65689: F, t72403: F) -> (F, F, F) {
    let t79008 = t78791 + t78792 + t78794 + t79006;
    let t79018 = t6224 * t6224;
    let t79024 = -t65628 / F::new(324.0) + t65632 / F::new(2304.0) + t5002 * t22246 / F::new(768.0) + t65647 / F::new(3456.0) + F::new(19.0) / F::new(288.0) * t6169 * t6164 * t488 - F::new(19.0) / F::new(1296.0) * t65664 - t15503 * t22271 / F::new(24.0) - t53336 * t22309 / F::new(24.0) + t1213 * t248 * t1214 * t79008 * t475 / F::new(3072.0) + t19083 * t6211 / F::new(36.0) + t72403 / F::new(72.0) + t65689 / F::new(1728.0) - F::new(3.0) / F::new(256.0) * t45030 * t248 * t1214 * t79018 * t11721;
    (t79008, t79018, t79024)
}
