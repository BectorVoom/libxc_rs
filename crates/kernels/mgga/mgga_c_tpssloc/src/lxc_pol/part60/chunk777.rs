//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 777/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk777<F: Float>(t794: F, t8479: F, t6897: F, t8537: F, t6562: F, t2053: F, t2717: F, t857: F, t6547: F, t8538: F, t30697: F, t30704: F, t30721: F, t2047: F, t214: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31198 = t794 * t8479;
    let t31200 = 0.82246703342411321825e-2 * t6897 * t31198;
    let t31319 = t794 * t8537;
    let t31320 = t6562 * t31319;
    let t31321 = 0.41123351671205660912e-2 * t31320;
    let t31332 = t2717 * t2053;
    let t31337 = t857 * t2053;
    let t31349 = t6547 * t8538;
    let t31350 = 0.19190897446562641759e-1 * t31349;
    let t31353 = 0.11304371706359309439e-1 * t30697;
    let t31355 = 0.26915170729426927235e-3 * t30704;
    let t31359 = 7.0 / 1152.0 * t30721;
    let t31366 = t214 * t2047;
    (t31198, t31200, t31319, t31321, t31332, t31337, t31350, t31353, t31355, t31359, t31366)
}
