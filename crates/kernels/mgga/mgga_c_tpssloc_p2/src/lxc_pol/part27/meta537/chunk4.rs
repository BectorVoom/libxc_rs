//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1961/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1961<F: Float>(t6880: F, t7685: F, t1266: F, t1976: F, t1980: F, t26002: F, t26005: F, t26006: F, t26098: F, t26138: F, t26141: F, t26144: F, t26145: F, t26147: F, t26150: F, t4026: F, t510: F, t5361: F, t574: F, t7451: F) -> F {
    let t26153 = F::cast_from(3.0_f64) * t7685 * t6880;
    let t26155 = -t1266 * t7451 - t1976 * t4026 + t1980 * t5361 - t26098 * t510 + t26138 * t574 - t26002 - t26005 - t26006 - t26141 - t26144 - t26145 + t26147 - t26150 + t26153;
    t26155
}
