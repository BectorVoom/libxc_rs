//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1867/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1867<F: Float>(t1561: F, t2860: F, t10760: F, t13517: F, t13519: F, t13522: F, t13524: F, t13526: F, t13657: F, t14263: F, t14266: F, t14271: F, t1569: F, t2863: F, t2881: F, t2889: F, t2907: F, t4411: F, t933: F) -> (F, F) {
    let t14276 = t1561 * t2860;
    let t14279 = -F::cast_from(0.11696447245269292414e1_f64) * t14263 * t2907 - t13517 - t13519 - t13522 - t13524 - t13526 - t13657 + F::new(2.0) * t14266 * t933 + F::new(1.0) * t4411 * t2881 + F::cast_from(0.32163958997385070134e2_f64) * t14271 * t2889 + F::new(1.0) * t10760 * t1569 - F::new(2.0) * t14276 * t2863;
    (t14276, t14279)
}
