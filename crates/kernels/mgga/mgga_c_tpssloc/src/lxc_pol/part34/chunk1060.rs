//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1060/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1060<F: Float>(t109: F, t22633: F, t28116: F, t90566: F, t22635: F, t26331: F, t26332: F, t6347: F, t20356: F, t6889: F, t6890: F, t80732: F, t1845: F, t1851: F, t5456: F, t106944: F, t106946: F, t106948: F, t84036: F, t86586: F, t96713: F, t96721: F) -> (F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t107460 = t22633 * t90566 * t28116;
    let t107464 = t26331 * t22635 * t26332 * t6347;
    let t107484 = t80732 * t6889 * t6890 * t20356;
    let t107504 = t6347 * t1845;
    let t107571 = t1851 * t5456;
    let t107634 = piecewise3(t110, 0.0, -t84036 - 22.0 / 3.0 * t86586 - 4.0 * t96713 + 2.0 * t96721 - 3.0 / 2.0 * t106944 + 3.0 / 2.0 * t106946 - t106948 / 4.0);
    (t107460, t107464, t107484, t107504, t107571, t107634)
}
