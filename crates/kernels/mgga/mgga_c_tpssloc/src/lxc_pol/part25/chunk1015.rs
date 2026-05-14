//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1015/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1015<F: Float>(t1888: F, t232: F, t40955: F, t6646: F, t23110: F, t23176: F, t23185: F, t252: F, t9660: F, t131: F, t2587: F, t81142: F, t1905: F, t9537: F, t23004: F, t22987: F, t25038: F, t25248: F, t2553: F) -> (F, F, F, F, F, F, F, F) {
    let t81667 = t1888 * t6646 * t40955 * t232;
    let t81670 = t23185 * t23110 * t23176;
    let t81672 = t252 * t9660;
    let t81675 = t1888 * t6646 * t81672 * t232;
    let t81686 = t81142 * t2587 * t131;
    let t81688 = t81686 * t9537 * t1905;
    let t81691 = t23185 * t23110 * t23004;
    let t81695 = t25038 * t25248 * t22987 * t2553;
    (t81667, t81670, t81672, t81675, t81686, t81688, t81691, t81695)
}
