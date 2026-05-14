//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1030/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1030<F: Float>(t14255: F, t291: F, t10629: F, t1580: F, t10632: F, t2906: F, t959: F, t1573: F, t2904: F, t4408: F, t923: F, t1561: F, t2885: F, t2860: F, t10760: F, t13517: F, t13519: F, t13522: F, t13524: F, t13526: F, t13657: F, t1569: F, t2863: F, t2881: F, t2889: F, t2907: F, t4411: F, t933: F) -> (F, F, F) {
    let t14257 = 0.621814e-1 * t14255 * t291;
    let t14258 = t10629 * t1580;
    let t14259 = t10632 * t2906;
    let t14260 = t14258 * t14259;
    let t14262 = 0.10254018858216406658e4 * t959 * t14260;
    let t14263 = t1573 * t2904;
    let t14266 = t4408 * t923;
    let t14271 = t1561 * t2885;
    let t14276 = t1561 * t2860;
    let t14279 = -0.11696447245269292414e1 * t14263 * t2907 - t13517 - t13519 - t13522 - t13524 - t13526 - t13657 + 2.0 * t14266 * t933 + 1.0 * t4411 * t2881 + 0.32163958997385070134e2 * t14271 * t2889 + 1.0 * t10760 * t1569 - 2.0 * t14276 * t2863;
    (t14257, t14262, t14279)
}
