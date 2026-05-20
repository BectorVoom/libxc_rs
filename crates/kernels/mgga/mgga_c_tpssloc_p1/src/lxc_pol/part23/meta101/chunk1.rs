//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 565/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk565<F: Float>(t2375: F, t3684: F, t1294: F, t2371: F, t2528: F, t2535: F, t570: F) -> (F, F, F, F, F, F) {
    let t3686 = F::cast_from(0.10843581300301739842e-1_f64) * t3684 * t2375;
    let t3688 = F::cast_from(0.11696447245269292414e1_f64) * t1294 * t2371;
    let t3690 = F::cast_from(0.17315859105681463759e2_f64) * t1294 * t2528;
    let t3695 = F::cast_from(0.5848223622634646207e0_f64) * t1294 * t2535;
    let t3700 = t570 * t570;
    let t3701 = F::new(1.0) / t3700;
    (t3686, t3688, t3690, t3695, t3700, t3701)
}
