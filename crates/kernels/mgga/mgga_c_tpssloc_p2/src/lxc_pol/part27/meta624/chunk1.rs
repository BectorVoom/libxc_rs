//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2105/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2105<F: Float>(t22473: F, t86595: F, t1453: F, t2358: F, t12808: F, t6530: F, t81438: F, t81443: F, t81445: F, t86583: F, t86586: F, t86589: F, t86591: F, t86593: F) -> F {
    let t86596 = t22473 * t86595;
    let t86598 = t1453 * t2358;
    let t86599 = t22473 * t86598;
    let t86601 = t6530 * t12808;
    let t86603 = -t81438 - t86583 - F::new(2.0) / F::new(3.0) * t81443 + t81445 / F::new(3.0) - F::new(11.0) / F::new(9.0) * t86586 - t86589 + t86591 - F::new(3.0) / F::new(4.0) * t86593 + t86596 / F::new(2.0) + t86599 / F::new(4.0) - t86601 / F::new(8.0);
    t86603
}
