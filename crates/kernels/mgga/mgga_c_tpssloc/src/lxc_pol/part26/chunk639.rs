//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 639/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk639<F: Float>(t2375: F, t3684: F, t1294: F, t2371: F, t2528: F, t1284: F, t172: F, t763: F, t2535: F, t184: F, t3681: F, t17: F) -> (F, F, F, F, F, F, F, F) {
    let t3686 = F::new(0.10843581300301739842e-1) * t3684 * t2375;
    let t3688 = F::new(0.11696447245269292414e1) * t1294 * t2371;
    let t3690 = F::new(0.17315859105681463759e2) * t1294 * t2528;
    let t3691 = t1284 * t172;
    let t3692 = t3691 * t763;
    let t3693 = F::new(0.11696447245269292414e1) * t3692;
    let t3695 = F::new(0.5848223622634646207e0) * t1294 * t2535;
    let t3696 = t3681 * t184;
    let t3697 = t17 * t3696;
    (t3686, t3688, t3690, t3691, t3693, t3695, t3696, t3697)
}
