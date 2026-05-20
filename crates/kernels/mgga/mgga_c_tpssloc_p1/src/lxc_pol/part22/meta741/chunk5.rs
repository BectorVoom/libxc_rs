//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2448/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2448<F: Float>(t135: F, t21446: F, t973: F, t41863: F, t48097: F, t48103: F, t68452: F, t68454: F, t68460: F, t68464: F, t68468: F, t68472: F, t68500: F, t68502: F, t68504: F, t68506: F, t68515: F, t68518: F, t68523: F, t68527: F, t68530: F, t68536: F, t68541: F) -> (F, F) {
    let t69579 = t973 * t135 * t21446;
    let t69598 = t48097 + F::new(2.0) / F::new(3.0) * t68452 - t68454 / F::new(9.0) - F::new(40.0) / F::new(27.0) * t48103 - t68460 - t68464 + t68468 / F::new(6.0) + t68472 / F::new(6.0) - F::new(40.0) / F::new(81.0) * t41863 - F::new(4.0) / F::new(81.0) * t68500 - t68502 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t68504 + F::new(2.0) / F::new(9.0) * t68506 + F::new(3.0) * t68515 - t68518 - F::new(8.0) / F::new(9.0) * t68523 + F::new(2.0) / F::new(9.0) * t68527 + F::new(14.0) / F::new(81.0) * t68530 - t68536 / F::new(3.0) + t68541 / F::new(18.0);
    (t69579, t69598)
}
