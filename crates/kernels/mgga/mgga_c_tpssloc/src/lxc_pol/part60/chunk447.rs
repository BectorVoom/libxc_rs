//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 447/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk447<F: Float>(t300: F, t5797: F, t5770: F, t1589: F, t4483: F, t2904: F, t5774: F, t951: F, t959: F, t5790: F, t942: F, t2929: F, t2932: F, t2980: F, t5392: F, t2979: F) -> (F, F, F, F, F, F, F) {
    let t5798 = t300 * t5797;
    let t5800 = 0.19751673498613801407e-1 * t300 * t5770;
    let t5802 = 0.11696447245269292414e1 * t4483 * t1589;
    let t5804 = t2904 * t5774 * t951;
    let t5806 = 0.11696447245269292414e1 * t959 * t5804;
    let t5808 = t942 * t5790 * t951;
    let t5810 = 0.5848223622634646207e0 * t959 * t5808;
    let t5811 = t2929 * t5774;
    let t5812 = t5811 * t2932;
    let t5814 = 0.17315859105681463759e2 * t959 * t5812;
    let t5817 = t2980 * t5392;
    let t5818 = t2979 * t5817;
    (t5798, t5800, t5802, t5806, t5810, t5814, t5818)
}
