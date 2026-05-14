//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 951/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk951<F: Float>(t12813: F, t510: F, t1458: F, t3652: F, t4098: F, t751: F, t2752: F, t4303: F, t172: F, t4095: F, t763: F, t1472: F, t2517: F, t1409: F, t9427: F, t2433: F, t3966: F) -> (F, F, F, F, F, F, F, F) {
    let t12835 = t510 * t12813;
    let t12841 = t3652 * t1458;
    let t12850 = 2.0 * t4098 * t751;
    let t12854 = t4303 * t2752;
    let t12858 = t4095 * t172;
    let t12860 = 0.11696447245269292414e1 * t12858 * t763;
    let t12861 = t1472 * t2517;
    let t12862 = t9427 * t1409;
    let t12865 = t2433 * t3966;
    (t12835, t12841, t12850, t12854, t12860, t12861, t12862, t12865)
}
