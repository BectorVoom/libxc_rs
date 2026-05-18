//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1091/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1091<F: Float>(t4034: F, t6535: F, t107: F, t240: F, t625: F, t656: F, t666: F, t2331: F, t63: F, t2332: F, t2358: F, t6530: F) -> (F, F, F, F, F, F, F) {
    let t22467 = F::new(4.0) * t4034 * t6535;
    let t22468 = t240 * t107;
    let t22469 = F::new(11.0) / F::new(9.0) * t22468;
    let t22470 = t625 * t656;
    let t22471 = t22470 * t666;
    let t22472 = F::new(2.0) / F::new(3.0) * t22471;
    let t22473 = t63 * t2331;
    let t22474 = t22473 * t2332;
    let t22476 = t6530 * t2358;
    (t22467, t22469, t22470, t22472, t22473, t22474, t22476)
}
