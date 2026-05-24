//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1349/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1349<F: Float>(t65564: F, t65567: F, t67143: F, t67150: F, t69510: F, t69512: F, t69515: F, t69517: F, t69519: F, t69521: F, t69523: F, t69525: F, t69527: F) -> F {
    let t71787 = t69510 / F::new(96.0) + t69512 / F::new(96.0) - t67143 - t65564 + t69515 / F::new(192.0) - F::new(7.0) / F::new(144.0) * t69517 + t69519 / F::new(384.0) + t69521 / F::new(192.0) - t69523 / F::new(384.0) - F::new(35.0) / F::new(54.0) * t65567 - F::new(7.0) / F::new(24.0) * t69525 + F::new(7.0) / F::new(72.0) * t69527 + t67150;
    t71787
}
