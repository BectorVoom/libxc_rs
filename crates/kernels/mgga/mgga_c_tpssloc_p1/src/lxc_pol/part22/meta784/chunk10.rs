//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2701/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2701<F: Float>(t4025: F, t5456: F, t20193: F, t604: F, t1411: F, t1434: F, t19322: F, t19363: F, t19441: F, t20207: F, t20264: F, t20285: F, t3962: F, t3966: F, t3968: F, t3971: F, t3976: F, t5398: F, t5442: F, t55653: F, t608: F, t609: F, t65: F, t6509: F, t67: F, t80: F) -> (F, F, F) {
    let t75275 = t4025 * t5456;
    let t75284 = t20193 * t604;
    let t75356 = -t608 * t20264 * t80 / F::new(12.0) - t19363 * t1434 / F::new(4.0) - t3976 * t5442 / F::new(4.0) - t609 * t20285 / F::new(12.0) - t3962 * t5442 / F::new(4.0) - t3968 * t5442 / F::new(4.0) - t3971 * t5442 / F::new(4.0) - t1411 * t19441 / F::new(4.0) - t3966 * t65 * t67 * t20207 / F::new(4.0) - t55653 * t20207 / F::new(4.0) - t19322 * t6509 * t5398 / F::new(4.0);
    (t75275, t75284, t75356)
}
