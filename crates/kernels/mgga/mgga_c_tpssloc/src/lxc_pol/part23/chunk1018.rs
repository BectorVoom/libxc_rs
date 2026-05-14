//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1018/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1018<F: Float>(t1256: F, t1763: F, t19267: F, t193: F, t21956: F, t21958: F, t21960: F, t21963: F, t21990: F, t22224: F, t22226: F, t22231: F, t22235: F, t22239: F, t22241: F, t22408: F, t336: F, t4700: F) -> (F,) {
    let t22412 = t1256 * t193 * t22408 * t336 - 3.0 * t1763 * t19267 * t4700 + t21956 + t21958 + t21960 - t21963 - t21990 - t22224 - t22226 + t22231 - t22235 - t22239 - t22241;
    (t22412,)
}
