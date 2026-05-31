//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 635/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk635<F: Float>(t2245: F, t2252: F, t2255: F, t2284: F, t2304: F, t609: F, t629: F, t642: F, t66: F, t80: F) -> F {
    let t2307 = -t2245 * t80 / F::cast_from(12.0_f64) - t2252 * t80 / F::cast_from(12.0_f64) - t2255 * t80 / F::cast_from(6.0_f64) - t609 * t642 / F::cast_from(6.0_f64) + t2284 * t80 / F::cast_from(24.0_f64) + t629 * t642 / F::cast_from(12.0_f64) + t66 * t2304 / F::cast_from(24.0_f64);
    t2307
}
