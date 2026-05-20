//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2710/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2710<F: Float>(t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39309: F, t39312: F, t56092: F, t56093: F, t56094: F, t56098: F, t56100: F, t56103: F, t56105: F, t56114: F, t56115: F, t56119: F) -> F {
    let t57193 = -t56092 - t56093 + t56094 + t56098 - t56100 - t56103 - t56105 - t39249 - t39256 + t56114 - t56115 - t39261 - t39266 - t39304 + t56119 - t39309 + t39312;
    t57193
}
