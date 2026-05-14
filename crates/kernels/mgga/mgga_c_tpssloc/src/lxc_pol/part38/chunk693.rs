//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 693/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk693<F: Float>(t25: F, t2375: F, t3684: F, t1294: F, t2371: F, t2528: F, t1284: F, t172: F, t763: F, t2535: F, t184: F, t3681: F, t17: F, t1388: F, t570: F, t515: F, t1298: F, t2249: F, t3665: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t3686 = 0.10843581300301739842e-1 * t3684 * t2375;
    let t3688 = 0.11696447245269292414e1 * t1294 * t2371;
    let t3690 = 0.17315859105681463759e2 * t1294 * t2528;
    let t3691 = t1284 * t172;
    let t3692 = t3691 * t763;
    let t3693 = 0.11696447245269292414e1 * t3692;
    let t3695 = 0.5848223622634646207e0 * t1294 * t2535;
    let t3696 = t3681 * t184;
    let t3697 = t17 * t3696;
    let t3698 = t1388 * t1388;
    let t3700 = t570 * t570;
    let t3701 = 1.0 / t3700;
    let t3704 = 1.0 / t515;
    let t3710 = piecewise3(t26, 0.0, -2.0 / 9.0 * t3704 * t3665 + 2.0 / 3.0 * t1298 * t2249);
    (t3686, t3688, t3690, t3691, t3692, t3693, t3695, t3696, t3697, t3698, t3700, t3701, t3704, t3710)
}
