//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1295/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1295<F: Float>(t120973: F, t120975: F, t120979: F, t120986: F, t120991: F, t120993: F, t24932: F, t27150: F, t27171: F, t27293: F, t31832: F, t32350: F, t4077: F, t652: F, t7042: F, t7056: F, t7266: F, t7802: F, t7806: F, t7904: F, t8103: F) -> (F,) {
    let t124918 = -2.0 * t652 * t7056 * t8103 - 2.0 * t24932 * t7802 - 2.0 * t24932 * t7806 - 2.0 * t27150 * t7266 - 2.0 * t27171 * t7266 - 2.0 * t27293 * t7042 + 3.0 * t31832 * t7904 - 2.0 * t32350 * t4077 - t120973 + t120975 - t120979 - t120986 + t120991 - t120993;
    (t124918,)
}
