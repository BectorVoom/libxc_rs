//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2529/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2529<F: Float>(t1099: F, t1118: F, t51147: F, t51159: F, t51173: F, t51186: F, t51200: F, t51212: F, t51226: F, t51239: F, t3263: F, t4737: F) -> (F, F) {
    let t51245 = F::cast_from(1.0_f64) * t1099 * (t51147 + t51159 + t51173 + t51186 + t51200 + t51212 + t51226 + t51239) * t1118;
    let t51246 = t4737 * t3263;
    (t51245, t51246)
}
