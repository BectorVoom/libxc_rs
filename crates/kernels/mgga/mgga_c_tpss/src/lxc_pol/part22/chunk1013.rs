//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1013/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1013<F: Float>(t11999: F, t434: F, t294: F, t3017: F, t4192: F, t3013: F, t3009: F, t4202: F, t4155: F, t1091: F, t3154: F, t4325: F, t11844: F, t11873: F, t11857: F, t11860: F, t11862: F, t11865: F, t11867: F, t11871: F, t11875: F, t11880: F, t11885: F, t11890: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12000 = t11999 * t434;
    let t12002 = 0.19751673498613801407e-1 * t294 * t12000;
    let t12004 = 0.5848223622634646207e0 * t4192 * t3017;
    let t12006 = 0.11696447245269292414e1 * t4192 * t3013;
    let t12008 = 0.11696447245269292414e1 * t3009 * t4202;
    let t12009 = t294 * t4155;
    let t12011 = 0.11696447245269292414e1 * t12009 * t1091;
    let t12012 = t4325 * t3154;
    let t12024 = 0.13892666666666666667e0 * t11844;
    let t12035 = 0.22954444444444444444e0 * t11873;
    let t12040 = -0.157790625e0 * t11857 - 0.3529725e1 * t11860 - 0.17648625e1 * t11862 + 0.6311625e0 * t11865 + 0.31558125e0 * t11867 + 0.62517e0 * t11871 + t12035 - 0.68863333333333333333e0 * t11875 + 0.57386111111111111112e0 * t11880 - 0.20659e1 * t11885 - 0.68863333333333333334e0 * t11890;
    (t12000, t12002, t12004, t12006, t12008, t12011, t12012, t12024, t12040)
}
