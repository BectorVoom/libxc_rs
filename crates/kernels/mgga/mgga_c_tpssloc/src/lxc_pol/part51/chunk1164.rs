//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1164/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1164<F: Float>(t131: F, t8511: F, t9239: F, t113875: F, t1862: F, t31680: F, t22573: F, t8606: F, t111: F, t8646: F, t112: F, t31781: F, t580: F, t1404: F, t2022: F, t7240: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t115894 = t8511 * t131;
    let t115895 = t9239 * t115894;
    let t115903 = t113875 * t1862;
    let t115907 = t9239 * t31680;
    let t115925 = t8606 * t22573;
    let t115984 = t8646 * t111;
    let t115996 = t31781 * t112;
    let t116014 = t31781 * t580;
    let t116021 = t8646 * t1404;
    let t116026 = t2022 * t7240;
    (t115894, t115895, t115903, t115907, t115925, t115984, t115996, t116014, t116021, t116026)
}
