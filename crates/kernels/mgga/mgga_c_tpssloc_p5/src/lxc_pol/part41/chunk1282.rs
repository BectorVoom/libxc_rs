//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1282/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1282<F: Float>(t5484: F, t8184: F, t29903: F, t30048: F, t30279: F, t30291: F, t30301: F, t30507: F, t30510: F, t30514: F, t30517: F, t30521: F, t30524: F, t30527: F, t64: F, t8128: F, t8137: F) -> (F, F) {
    let t30530 = t8184 * t5484;
    let t30533 = -t30048 - F::new(4.0) / F::new(3.0) * t30279 - F::new(10.0) / F::new(9.0) * t30291 + F::new(10.0) / F::new(9.0) * t30301 - F::new(3.0) / F::new(4.0) * t29903 * t30507 - F::new(5.0) / F::new(6.0) * t8128 * t30510 + F::new(5.0) / F::new(6.0) * t8128 * t30514 + t8128 * t30517 / F::new(4.0) - F::new(5.0) / F::new(9.0) * t64 * t30521 + F::new(25.0) / F::new(36.0) * t8137 * t30524 - F::new(5.0) / F::new(36.0) * t8137 * t30527 - F::new(5.0) / F::new(24.0) * t8137 * t30530;
    (t30530, t30533)
}
