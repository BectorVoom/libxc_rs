//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 399/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk399<F: Float>(t265: F, t394: F, t1068: F, t1070: F, t193: F, t336: F, t873: F, t890: F, t916: F, t956: F, t958: F, t963: F) -> F {
    let t395 = t265 < t394;
    let t1074 = piecewise3::<F>(t395, t1068 * t1070 * t193 * t336 - t890 + t916 + t956 + t958 - t963, t873);
    t1074
}
