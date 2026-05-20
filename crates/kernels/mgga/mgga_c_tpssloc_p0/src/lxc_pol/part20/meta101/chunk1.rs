//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 681/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk681<F: Float>(t153: F, t2517: F, t145: F, t2447: F, t185: F, t193: F, t2373: F, t2377: F, t2378: F, t2379: F, t2408: F, t2417: F, t2423: F, t2426: F, t2429: F, t2432: F, t2450: F) -> (F, F, F, F) {
    let t2518 = t153 * t2517;
    let t2519 = t145 * t2447;
    let t2520 = t2519 * t185;
    let t2521 = F::new(6.0) * t193 * t2378 * t2379 + t2373 + t2377 + t2408 + t2417 - t2423 - t2426 + t2429 + t2432 + t2450 + t2518 + t2520;
    (t2518, t2519, t2520, t2521)
}
