//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2138/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2138<F: Float>(t10224: F, t2995: F, t973: F, t10228: F, t2960: F, t10225: F, t10213: F, t135: F, t10218: F, t10236: F, t10913: F, t41961: F) -> (F, F, F, F, F, F, F) {
    let t42962 = t973 * t10224 * t2995;
    let t42964 = t2960 * t10228;
    let t42968 = t2960 * t10225;
    let t42972 = t135 * t10213;
    let t42974 = t973 * t42972 * t10218;
    let t42985 = t10236 * t10913;
    let t43002 = F::new(220.0) / F::new(81.0) * t41961;
    (t42962, t42964, t42968, t42972, t42974, t42985, t43002)
}
