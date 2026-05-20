//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1010/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1010<F: Float>(t1985: F, t24138: F, t6889: F, t6906: F, t22685: F, t22686: F, t31611: F, t22934: F, t2085: F, t3791: F, t1992: F, t550: F, t6976: F) -> (F, F, F, F, F) {
    let t115368 = t1985 * t6889 * t6906 * t24138;
    let t115372 = t22685 * t31611 * t22686;
    let t115378 = t1985 * t31611 * t22934;
    let t115384 = t2085 * t3791;
    let t115387 = t1992 * t6976 * t115384 * t550;
    (t115368, t115372, t115378, t115384, t115387)
}
