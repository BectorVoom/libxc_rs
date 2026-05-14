//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 436/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk436<F: Float>(t2528: F, t761: F, t172: F, t753: F, t763: F, t2504: F, t739: F, t746: F) -> (F, F, F, F) {
    let t2530 = 0.17315859105681463759e2 * t761 * t2528;
    let t2531 = t753 * t172;
    let t2532 = t2531 * t763;
    let t2533 = 0.11696447245269292414e1 * t2532;
    let t2535 = t739 * t2504 * t746;
    (t2530, t2531, t2533, t2535)
}
