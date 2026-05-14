//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1276/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1276<F: Float>(t1673: F, t6061: F, t1275: F, t6556: F, t1659: F, t4519: F, t19579: F, t19580: F, t18547: F, t19609: F, t24790: F, t1760: F, t21107: F, t5757: F, t21112: F, t5706: F) -> (F, F, F, F, F, F) {
    let t68786 = 2.0 * t6061 * t1673;
    let t68788 = 2.0 * t1275 * t6556;
    let t68798 = t1659 * t4519;
    let t68801 = 4.0 * t19579 * t19580 * t68798;
    let t68808 = 6.0 * t18547 * t24790 * t19609;
    let t68810 = t1760 * t21107 * t5757;
    let t68814 = 6.0 * t5706 * t21112;
    (t68786, t68788, t68801, t68808, t68810, t68814)
}
