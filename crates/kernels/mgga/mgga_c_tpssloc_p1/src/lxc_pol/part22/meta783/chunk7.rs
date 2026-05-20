//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2686/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2686<F: Float>(t1352: F, t16233: F, t16305: F, t16394: F, t19886: F, t19894: F, t19981: F, t3803: F, t40449: F, t54013: F, t54014: F, t54786: F, t54793: F, t54812: F, t56812: F, t57091: F, t57437: F, t57450: F, t57457: F, t6394: F, t74415: F) -> F {
    let t74833 = -t3803 * t54013 * t74415 * t1352 / F::new(1024.0) + t3803 * t16305 * t57091 * t6394 / F::new(256.0) - F::new(3.0) / F::new(512.0) * t16233 * t54013 * t54014 * t56812 - F::new(5.0) / F::new(256.0) * t16394 * t19981 - F::new(7.0) / F::new(384.0) * t57437 + F::new(7.0) / F::new(96.0) * t57450 + F::new(7.0) / F::new(768.0) * t57457 + t54786 - F::new(595.0) / F::new(3456.0) * t54793 + t40449 + t54812 + t3803 * t16305 * t56812 * t6394 / F::new(256.0) - F::new(5.0) / F::new(128.0) * t16394 * t19894 + t16394 * t19886 / F::new(128.0);
    t74833
}
