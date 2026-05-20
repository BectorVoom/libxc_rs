//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2652/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2652<F: Float>(t1409: F, t628: F, t67: F, t2250: F, t5398: F, t16558: F, t607: F, t12606: F, t12620: F, t12623: F, t12662: F, t12665: F, t1411: F, t1434: F, t17635: F, t1864: F, t19322: F, t19323: F, t19363: F, t19404: F, t2251: F, t3966: F, t3968: F, t3971: F, t4018: F, t5427: F, t608: F, t642: F, t65: F, t6509: F, t80: F) -> (F, F, F) {
    let t55653 = t1409 * t628 * t67;
    let t55662 = t2250 * t5398;
    let t55666 = t607 * t16558;
    let t55673 = -t12662 * t1434 / F::new(6.0) - t12665 * t1434 / F::new(3.0) - t3968 * t4018 / F::new(3.0) - t12623 * t1434 / F::new(6.0) - t3971 * t4018 / F::new(3.0) - t1411 * t12620 / F::new(6.0) - t2251 * t5427 * t80 / F::new(12.0) - t608 * t19404 * t80 / F::new(6.0) - t19363 * t642 / F::new(6.0) - t55653 * t19323 / F::new(3.0) - t19322 * t6509 * t3966 / F::new(3.0) - t19322 * t1864 * t12606 / F::new(6.0) - t55662 * t65 * t80 / F::new(12.0) - t55666 * t65 * t80 / F::new(6.0) - t17635 * t628 * t80 / F::new(6.0);
    (t55662, t55666, t55673)
}
