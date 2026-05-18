//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 918/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk918<F: Float>(t2035: F, t2363: F, t2094: F, t40611: F, t12461: F, t7216: F, t193: F, t7125: F, t2053: F, t40889: F, t10109: F, t7106: F) -> (F, F, F, F, F, F) {
    let t91857 = t2035 * t2363;
    let t92169 = t2094 * t40611;
    let t92200 = t7216 * t12461;
    let t92271 = t193 * t7125;
    let t92394 = t40889 * t2053;
    let t92981 = t10109 * t7106;
    (t91857, t92169, t92200, t92271, t92394, t92981)
}
