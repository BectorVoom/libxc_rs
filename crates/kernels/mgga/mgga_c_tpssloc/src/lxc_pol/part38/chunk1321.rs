//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1321/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1321<F: Float>(t110143: F, t8226: F, t110076: F, t110078: F, t110080: F, t110089: F, t110102: F, t110103: F, t110105: F, t110503: F, t110506: F, t12808: F, t1453: F, t8128: F, t8129: F) -> F {
    let t110510 = t110143 * t8226;
    let t110517 = t8128 * t8129 * t12808 / F::new(4.0) + F::new(22.0) / F::new(9.0) * t110503 + t110506 + F::new(10.0) / F::new(9.0) * t8128 * t110089 * t1453 - F::new(55.0) / F::new(27.0) * t110510 + F::new(2.0) * t110076 + F::new(20.0) / F::new(9.0) * t110078 + F::new(10.0) / F::new(27.0) * t110080 + t110102 + F::new(110.0) / F::new(27.0) * t110103 + F::new(40.0) / F::new(27.0) * t110105;
    t110517
}
