//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2019/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2019<F: Float>(t26129: F, t81442: F, t22470: F, t4067: F, t111: F, t7758: F, t112: F, t26509: F, t25: F, t40772: F, t1408: F, t2752: F) -> (F, F, F, F, F, F) {
    let t86588 = t81442 * t26129;
    let t86589 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t86588;
    let t86590 = t22470 * t4067;
    let t86591 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t86590;
    let t86647 = t7758 * t111;
    let t86656 = t26509 * t112;
    let t86716 = t40772 * t25;
    let t86721 = t2752 * t1408;
    (t86589, t86591, t86647, t86656, t86716, t86721)
}
