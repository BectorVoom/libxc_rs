//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1063/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1063<F: Float>(t4889: F, t4916: F, t1653: F, t7319: F, t4919: F, t15293: F, t4928: F, t8034: F, t4934: F, t1184: F, t460: F, t6144: F, t1178: F, t16558: F, t1177: F, t6138: F) -> (F, F, F, F, F, F, F) {
    let t18536 = t4889 * t4916;
    let t18542 = t7319 * t1653;
    let t18543 = t4919 * t18542;
    let t18546 = t4919 * t15293;
    let t18549 = t8034 * t4928;
    let t18550 = t4934 * t18549;
    let t18554 = t6144 * t1184 * t460;
    let t18555 = t4934 * t18554;
    let t18558 = t1178 * t16558;
    let t18559 = t1177 * t18558;
    let t18563 = t6138 * t1184 * t460;
    (t18536, t18543, t18546, t18550, t18555, t18559, t18563)
}
