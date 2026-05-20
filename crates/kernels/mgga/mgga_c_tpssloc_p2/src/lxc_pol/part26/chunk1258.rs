//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1258/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1258<F: Float>(t12823: F, t6535: F, t107: F, t835: F, t240: F, t656: F, t666: F, t2331: F, t625: F, t2332: F, t22470: F, t2358: F) -> (F, F, F, F, F) {
    let t81434 = F::new(6.0) * t12823 * t6535;
    let t81437 = t835 * t107;
    let t81438 = F::new(154.0) / F::new(27.0) * t81437;
    let t81439 = t240 * t656;
    let t81440 = t81439 * t666;
    let t81442 = t625 * t2331;
    let t81443 = t81442 * t2332;
    let t81445 = t22470 * t2358;
    (t81434, t81438, t81440, t81443, t81445)
}
