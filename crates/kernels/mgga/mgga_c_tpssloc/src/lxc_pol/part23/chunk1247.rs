//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1247/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1247<F: Float>(t21095: F, t4483: F, t48103: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t68494: F, t68498: F, t68500: F, t77028: F, t77030: F, t77032: F, t77034: F) -> (F, F) {
    let t77159 = 0.4155806185363551302e3 * t4483 * t21095;
    let t77174 = 0.24154e1 * t68442 + 0.40256666666666666668e0 * t68444 + 0.44729629629629629629e0 * t68446 - 0.16102666666666666667e1 * t68448 - 0.132456e1 * t68452 + 0.22076e0 * t68454 + 0.98115555555555555556e0 * t48103 + 0.80513333333333333333e0 * t68494 - 0.24154e1 * t68498 + 0.11651625e2 * t77028 - 0.51785e1 * t77030 - 0.247573125e0 * t77032 + 0.3300975e0 * t77034 + 0.98115555555555555555e-1 * t68500;
    (t77159, t77174)
}
