//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1101/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1101<F: Float>(t2274: F, t5392: F, t5398: F, t55: F, t1420: F, t1423: F, t2282: F, t39: F, t51: F, t5408: F, t5411: F, t5416: F, t56: F) -> (F, F, F) {
    let t5421 = t2274 * t5392;
    let t5424 = t55 * t5398;
    let t5427 = F::new(5.0) / F::new(18.0) * t39 * t5408 + F::new(5.0) / F::new(6.0) * t39 * t5411 + F::new(88.0) / F::new(9.0) * t5416 * t56 + F::new(40.0) / F::new(9.0) * t1420 * t1423 + F::new(5.0) / F::new(18.0) * t51 * t5421 - F::new(5.0) / F::new(6.0) * t51 * t5424 - t2282;
    (t5421, t5424, t5427)
}
