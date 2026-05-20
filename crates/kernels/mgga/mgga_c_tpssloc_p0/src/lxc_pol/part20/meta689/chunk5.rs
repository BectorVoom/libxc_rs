//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2616/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2616<F: Float>(t3577: F, t44951: F, t4949: F, t11692: F, t1227: F, t15615: F, t15702: F, t3578: F, t45049: F, t45114: F, t4582: F, t4728: F, t484: F, t48554: F, t488: F, t4978: F, t52462: F, t52897: F, t53135: F, t53142: F, t53144: F, t53149: F, t53155: F, t53158: F, t68: F) -> F {
    let t53161 = t3577 * t44951 * t4949;
    let t53162 = t53161 / F::new(6912.0);
    let t53167 = t52462 * t68 * t484 * t488 / F::new(3072.0) + t53135 / F::new(1152.0) - F::new(5.0) / F::new(20736.0) * t45049 - t1227 * t4582 * t15615 * t48554 / F::new(256.0) - t53142 / F::new(288.0) + t11692 * t3578 * t4728 * t53144 / F::new(768.0) + t11692 * t3578 * t53149 * t15702 / F::new(1536.0) - t53155 / F::new(2304.0) - t53158 / F::new(1152.0) + t53162 - F::new(3.0) / F::new(512.0) * t45114 * t52897 * t53149 * t4978;
    t53167
}
