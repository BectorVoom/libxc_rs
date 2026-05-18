//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 868/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk868<F: Float>(t6547: F, t8538: F, t798: F, t8543: F, t30697: F, t30704: F, t30721: F, t30701: F, t30707: F, t30710: F, t30717: F, t30723: F) -> (F, F, F) {
    let t31349 = t6547 * t8538;
    let t31350 = F::new(0.19190897446562641759e-1) * t31349;
    let t31351 = t798 * t8543;
    let t31353 = F::new(0.11304371706359309439e-1) * t30697;
    let t31355 = F::new(0.26915170729426927235e-3) * t30704;
    let t31359 = F::new(7.0) / F::new(1152.0) * t30721;
    let t31361 = -t31353 - F::new(0.96894614625936938046e-2) * t30701 - t31355 - F::new(0.16149102437656156341e-2) * t30707 + t30710 / F::new(768.0) - t30717 / F::new(768.0) - t31359 - t30723 / F::new(192.0);
    (t31350, t31351, t31361)
}
