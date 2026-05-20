//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2351/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2351<F: Float>(t2110: F, t24514: F, t26070: F, t26073: F, t26076: F, t27303: F, t27365: F, t27961: F, t27982: F, t7256: F, t7259: F, t7435: F, t7975: F, t85480: F, t85536: F, t96403: F, t96559: F, t96562: F) -> F {
    let t104942 = t96559 * t2110 / F::new(3.0) + t96562 * t2110 / F::new(3.0) + t27982 * t7256 / F::new(3.0) + t27982 * t7259 / F::new(3.0) - F::new(5.0) * t85536 * t27961 - F::new(5.0) * t85480 * t27961 - F::new(5.0) * t24514 * t96403 + F::new(2.0) / F::new(3.0) * t26070 * t7975 + F::new(2.0) / F::new(3.0) * t26073 * t7975 + F::new(2.0) / F::new(3.0) * t26076 * t7975 + F::new(2.0) / F::new(3.0) * t7435 * t27365 + F::new(2.0) / F::new(3.0) * t7435 * t27303;
    t104942
}
