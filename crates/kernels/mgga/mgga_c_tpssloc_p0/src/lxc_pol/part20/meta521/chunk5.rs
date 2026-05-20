//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2055/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2055<F: Float>(t2223: F, t3826: F, t11985: F, t25: F, t514: F, t11998: F, t28: F, t517: F, t12442: F, t225: F, t12036: F, t12016: F) -> (F, F, F, F, F, F) {
    let t39857 = t2223 * t3826;
    let t39861 = F::new(1.0) / t514 / t11985 / t25;
    let t39877 = F::new(1.0) / t517 / t11998 / t28;
    let t39910 = t12442 * t225;
    let t39913 = t12036 * t225;
    let t39916 = t12016 * t225;
    (t39857, t39861, t39877, t39910, t39913, t39916)
}
