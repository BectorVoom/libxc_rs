//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 826/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk826<F: Float>(t2053: F, t40889: F, t10143: F, t7844: F, t2091: F, t40590: F, t111: F, t7945: F, t8110: F, t7684: F, t8944: F, t225: F, t28051: F, t2006: F, t6387: F, t6414: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92394 = t40889 * t2053;
    let t93000 = t7844 * t10143;
    let t93319 = t40590 * t2091;
    let t94170 = t7945 * t111;
    let t96334 = t8110 * t111;
    let t96797 = t7684 * t8944;
    let t96913 = t28051 * t225;
    let t97172 = t2006 * t6387;
    let t97181 = t2006 * t6414;
    (t92394, t93000, t93319, t94170, t96334, t96797, t96913, t97172, t97181)
}
