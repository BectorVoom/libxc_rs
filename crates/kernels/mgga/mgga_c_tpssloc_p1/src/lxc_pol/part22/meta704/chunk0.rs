//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2292/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2292<F: Float>(t1222: F, t18982: F, t13969: F, t18947: F, t3506: F, t11719: F, t18302: F, t1174: F, t18225: F, t3431: F, t18221: F, t15522: F, t4889: F) -> (F, F, F, F, F, F) {
    let t66410 = t18982 * t1222;
    let t66413 = t3506 * t13969 * t18947;
    let t66437 = t11719 * t13969 * t18302;
    let t66449 = t1174 * t3431 * t18225;
    let t66452 = t1174 * t3431 * t18221;
    let t66458 = t4889 * t15522;
    (t66410, t66413, t66437, t66449, t66452, t66458)
}
