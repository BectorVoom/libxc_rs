//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1258/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1258<F: Float>(t12461: F, t7216: F, t193: F, t7125: F, t26739: F, t2752: F, t201: F, t7844: F, t225: F, t26722: F, t2053: F, t40889: F) -> (F, F, F, F, F, F) {
    let t92200 = t7216 * t12461;
    let t92271 = t193 * t7125;
    let t92276 = t26739 * t2752;
    let t92319 = t193 * t201 * t7844;
    let t92386 = t26722 * t225;
    let t92394 = t40889 * t2053;
    (t92200, t92271, t92276, t92319, t92386, t92394)
}
