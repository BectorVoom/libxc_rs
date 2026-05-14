//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1110/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1110<F: Float>(t12021: F, t16474: F, t12033: F, t1375: F, t1386: F, t16453: F, t16458: F, t16460: F, t16463: F, t16465: F, t16468: F, t16471: F, t1843: F, t3758: F, t3882: F, t3889: F, t5215: F, t5326: F, t5354: F, t568: F) -> (F,) {
    let t16475 = t12021 * t16474;
    let t16485 = -t12033 * t1843 + 4.0 * t1375 * t16453 + 2.0 * t1375 * t16471 - 6.0 * t1375 * t16475 - 2.0 * t1386 * t16460 + t16458 * t568 + t16463 * t568 + 2.0 * t16465 * t568 + t16468 * t568 + 4.0 * t3758 * t5326 + 4.0 * t3882 * t5326 - 2.0 * t3882 * t5354 + 2.0 * t3889 * t5215;
    (t16485,)
}
