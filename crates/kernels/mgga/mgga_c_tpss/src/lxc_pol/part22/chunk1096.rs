//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1096/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1096<F: Float>(t11999: F, t434: F, t294: F, t3017: F, t4192: F, t3013: F, t3009: F, t4202: F, t4155: F, t1091: F, t3154: F, t4325: F) -> (F, F, F, F, F, F, F) {
    let t12000 = t11999 * t434;
    let t12002 = F::new(0.19751673498613801407e-1) * t294 * t12000;
    let t12004 = F::new(0.5848223622634646207e0) * t4192 * t3017;
    let t12006 = F::new(0.11696447245269292414e1) * t4192 * t3013;
    let t12008 = F::new(0.11696447245269292414e1) * t3009 * t4202;
    let t12009 = t294 * t4155;
    let t12011 = F::new(0.11696447245269292414e1) * t12009 * t1091;
    let t12012 = t4325 * t3154;
    (t12000, t12002, t12004, t12006, t12008, t12011, t12012)
}
