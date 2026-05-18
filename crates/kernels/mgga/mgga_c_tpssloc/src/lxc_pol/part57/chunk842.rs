//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 842/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk842<F: Float>(t6547: F, t8538: F, t30697: F, t30704: F, t30721: F, t2047: F, t214: F) -> (F, F, F, F, F) {
    let t31349 = t6547 * t8538;
    let t31350 = F::new(0.19190897446562641759e-1) * t31349;
    let t31353 = F::new(0.11304371706359309439e-1) * t30697;
    let t31355 = F::new(0.26915170729426927235e-3) * t30704;
    let t31359 = F::new(7.0) / F::new(1152.0) * t30721;
    let t31366 = t214 * t2047;
    (t31350, t31353, t31355, t31359, t31366)
}
