//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2455/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2455<F: Float>(t50262: F, t10875: F, t48569: F, t10879: F, t10904: F, t13977: F, t13987: F, t14001: F, t14006: F, t2960: F, t42561: F, t43228: F, t43233: F, t47701: F, t50242: F, t50250: F, t50255: F, t50259: F, t973: F, t977: F) -> F {
    let t50263 = t50262 / F::new(6912.0);
    let t50265 = t48569 * t10875;
    let t50268 = t43228 / F::new(432.0) + t2960 * t14001 / F::new(9.0) - t50242 / F::new(72.0) + t973 * t977 * t47701 / F::new(16.0) + t2960 * t14006 / F::new(18.0) - t50250 / F::new(144.0) - t42561 * t13987 / F::new(32.0) + t50255 / F::new(256.0) + t50259 - t10904 * t13977 / F::new(48.0) - t50263 - t43233 / F::new(1536.0) - t50265 * t10879 / F::new(512.0);
    t50268
}
