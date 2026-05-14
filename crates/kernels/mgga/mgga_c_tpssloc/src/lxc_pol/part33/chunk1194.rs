//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1194/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1194<F: Float>(t22986: F, t25249: F, t5612: F, t6646: F, t1510: F, t98389: F, t98422: F, t20756: F, t6637: F, t6638: F, t81984: F, t1888: F, t22996: F, t2632: F, t67358: F, t1499: F, t20857: F, t20861: F, t20870: F, t23008: F, t28407: F, t28411: F, t4166: F, t6657: F, t812: F, t81689: F, t81717: F, t81991: F, t82047: F, t87635: F, t87653: F, t87666: F, t87718: F, t98564: F, t98884: F) -> (F,) {
    let t105661 = t22986 * t6646 * t25249 * t5612;
    let t105665 = t22986 * t6646 * t98389 * t1510;
    let t105669 = t22986 * t6646 * t98422 * t1510;
    let t105674 = t81984 * t6637 * t6638 * t20756;
    let t105685 = t1888 * t22996 * t67358 * t2632;
    let t105689 = 0.11514538467937585055e0 * t98564 + 3.0 * t1499 * t28407 + 6.0 * t812 * t23008 * t20861 - t81689 - 0.38381794893125283518e0 * t87635 - 0.24674011002723396547e-1 * t87653 + t81717 + 0.49348022005446793095e-1 * t105661 + 0.9869604401089358619e-1 * t105665 + 0.49348022005446793095e-1 * t105669 - 0.19190897446562641759e0 * t87666 - 0.19739208802178717238e0 * t105674 - 3.0 * t4166 * t28411 - t812 * t6657 * t20870 - 6.0 * t812 * t81991 * t20857 + 0.49348022005446793095e-1 * t105685 - t82047 - 0.15626873635058151147e0 * t87718 + 0.12337005501361698274e-1 * t98884;
    (t105689,)
}
