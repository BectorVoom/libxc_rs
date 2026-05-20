//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1766/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1766<F: Float>(t19440: F, t72: F, t1411: F, t1427: F, t1434: F, t19363: F, t19405: F, t3968: F, t3971: F, t3976: F, t3998: F, t4018: F, t5428: F, t5442: F, t609: F, t629: F, t642: F, t66: F, t80: F) -> (F, F) {
    let t19441 = t72 * t19440;
    let t19444 = -t3968 * t1434 / F::new(6.0) - t3971 * t1434 / F::new(6.0) - t1411 * t4018 / F::new(6.0) - t19363 * t80 / F::new(12.0) + t19405 * t80 / F::new(24.0) + t5428 * t642 / F::new(24.0) - t3976 * t1434 / F::new(6.0) + t3998 * t1434 / F::new(12.0) + t1427 * t4018 / F::new(12.0) - t609 * t5442 / F::new(12.0) + t629 * t5442 / F::new(24.0) + t66 * t19441 / F::new(24.0);
    (t19441, t19444)
}
