//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 654/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk654<F: Float>(t6889: F, t8621: F, t1985: F, t2015: F, t2091: F, t3887: F, t1998: F, t2085: F, t214: F, t553: F, t8617: F, t544: F, t8482: F) -> (F, F, F, F, F, F, F) {
    let t8622 = t6889 * t8621;
    let t8623 = t1985 * t8622;
    let t8627 = t3887 * t2091 * t2015;
    let t8630 = t1998 * t2085;
    let t8631 = t214 * t8630;
    let t8632 = t1985 * t8631;
    let t8634 = t553 * t8617;
    let t8636 = t8482 + F::cast_from(0.82246703342411321825e-2_f64) * t8632 + t544 * t8634;
    (t8622, t8623, t8627, t8630, t8631, t8634, t8636)
}
