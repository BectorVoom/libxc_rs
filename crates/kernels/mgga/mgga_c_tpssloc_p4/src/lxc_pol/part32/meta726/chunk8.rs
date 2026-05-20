//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2349/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2349<F: Float>(t2110: F, t26063: F, t26067: F, t27332: F, t27341: F, t27966: F, t7256: F, t7259: F, t7432: F, t95981: F, t96028: F, t96072: F, t96406: F, t96479: F, t96482: F) -> F {
    let t104885 = F::new(5.0) / F::new(3.0) * t96028 * t7432 + F::new(5.0) / F::new(3.0) * t27341 * t26063 + F::new(5.0) / F::new(3.0) * t27341 * t26067 + F::new(2.0) / F::new(3.0) * t96406 * t2110 + F::new(2.0) / F::new(3.0) * t96479 * t2110 + F::new(2.0) / F::new(3.0) * t96482 * t2110 + F::new(2.0) / F::new(3.0) * t27966 * t7256 + F::new(2.0) / F::new(3.0) * t27966 * t7259 + F::new(5.0) / F::new(3.0) * t95981 * t7432 + F::new(5.0) / F::new(3.0) * t96072 * t7432 + F::new(5.0) / F::new(3.0) * t27332 * t26063 + F::new(5.0) / F::new(3.0) * t27332 * t26067;
    t104885
}
