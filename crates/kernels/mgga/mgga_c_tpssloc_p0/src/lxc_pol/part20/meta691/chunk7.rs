//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2630/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2630<F: Float>(t3566: F, t5023: F, t15734: F, t3490: F, t11789: F, t1227: F, t248: F, t4733: F, t11814: F, t1232: F, t15498: F, t3527: F, t3531: F, t45264: F, t45266: F, t45271: F, t45283: F, t45296: F, t5014: F) -> F {
    let t53507 = t3566 * t5023;
    let t53515 = t3490 * t15734;
    let t53516 = t53515 / F::new(6912.0);
    let t53519 = t1227 * t248 * t11789 * t4733;
    let t53520 = t53519 / F::new(6912.0);
    let t53524 = -t45264 / F::new(2304.0) - t45266 / F::new(2304.0) - F::new(5.0) / F::new(7776.0) * t45271 + t53507 * t1232 / F::new(288.0) + t15498 * t3527 / F::new(288.0) + t15498 * t3531 / F::new(144.0) - t45283 / F::new(768.0) + t53516 + t53520 - t45296 / F::new(5184.0) + t11814 * t5014 / F::new(1024.0);
    t53524
}
