//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 999/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk999<F: Float>(t1985: F, t27114: F, t6889: F, t6906: F, t115296: F, t1799: F, t22633: F, t22635: F, t2086: F, t254: F, t33297: F, t6883: F) -> (F, F, F, F) {
    let t122192 = t1985 * t6889 * t6906 * t27114;
    let t122204 = t22633 * t22635 * t115296 * t1799;
    let t122206 = t2086 * t254;
    let t122210 = t6883 * t33297;
    (t122192, t122204, t122206, t122210)
}
