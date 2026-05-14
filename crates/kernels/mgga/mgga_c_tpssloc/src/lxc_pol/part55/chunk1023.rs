//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1023/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1023<F: Float>(t5052: F, t7299: F, t225: F, t27419: F, t27805: F, t27424: F, t27422: F, t111: F, t27370: F, t112: F, t27907: F, t8110: F, t1307: F, t1842: F, t1527: F, t776: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94558 = t7299 * t5052;
    let t94656 = t27419 * t225;
    let t95836 = t27805 * t225;
    let t95899 = t27424 * t225;
    let t95902 = t27422 * t225;
    let t96238 = t27370 * t111;
    let t96311 = t27907 * t112;
    let t96334 = t8110 * t111;
    let t97721 = t1842 * t1307;
    let t98960 = t1527 * t776;
    (t94558, t94656, t95836, t95899, t95902, t96238, t96311, t96334, t97721, t98960)
}
