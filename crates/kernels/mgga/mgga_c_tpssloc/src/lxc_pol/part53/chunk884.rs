//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 884/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk884<F: Float>(t2086: F, t254: F, t33297: F, t6883: F, t115545: F, t22633: F, t26338: F, t120240: F, t22635: F, t31558: F, t26331: F, t31549: F, t5308: F, t1985: F, t26193: F, t31607: F) -> (F, F, F, F, F, F) {
    let t122206 = t2086 * t254;
    let t122210 = t6883 * t33297;
    let t122213 = t22633 * t115545 * t26338;
    let t122218 = t22633 * t22635 * t31558 * t120240;
    let t122227 = t26331 * t22635 * t31549 * t5308;
    let t122235 = t1985 * t26193 * t31607;
    (t122206, t122210, t122213, t122218, t122227, t122235)
}
