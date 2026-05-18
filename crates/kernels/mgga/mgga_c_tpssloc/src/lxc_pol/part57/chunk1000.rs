//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1000/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1000<F: Float>(t127796: F, t127833: F, t127858: F, t127883: F, t127926: F, t127947: F, t128042: F, t128072: F, t870: F, t1530: F, t33476: F, t1914: F, t5544: F) -> (F, F, F, F) {
    let t128075 = t127796 + t127833 + t127858 + t127883 + t127926 + t127947 + t128042 + t128072;
    let t128076 = t128075 * t870;
    let t128080 = t33476 * t1530;
    let t128086 = t1914 * t5544;
    (t128075, t128076, t128080, t128086)
}
