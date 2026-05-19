//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1333/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1333<F: Float>(t105661: F, t105665: F, t105669: F, t105674: F, t105685: F, t1499: F, t20857: F, t20861: F, t20870: F, t23008: F, t28407: F, t28411: F, t4166: F, t6657: F, t812: F, t81689: F, t81717: F, t81991: F, t82047: F, t87635: F, t87653: F, t87666: F, t87718: F, t98564: F, t98884: F) -> F {
    let t105689 = F::cast_from(0.11514538467937585055e0_f64) * t98564 + F::new(3.0) * t1499 * t28407 + F::new(6.0) * t812 * t23008 * t20861 - t81689 - F::cast_from(0.38381794893125283518e0_f64) * t87635 - F::cast_from(0.24674011002723396547e-1_f64) * t87653 + t81717 + F::cast_from(0.49348022005446793095e-1_f64) * t105661 + F::cast_from(0.9869604401089358619e-1_f64) * t105665 + F::cast_from(0.49348022005446793095e-1_f64) * t105669 - F::cast_from(0.19190897446562641759e0_f64) * t87666 - F::cast_from(0.19739208802178717238e0_f64) * t105674 - F::new(3.0) * t4166 * t28411 - t812 * t6657 * t20870 - F::new(6.0) * t812 * t81991 * t20857 + F::cast_from(0.49348022005446793095e-1_f64) * t105685 - t82047 - F::cast_from(0.15626873635058151147e0_f64) * t87718 + F::cast_from(0.12337005501361698274e-1_f64) * t98884;
    t105689
}
