//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2323/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2323<F: Float>(t25927: F, t98030: F, t23788: F, t98011: F, t1081: F, t5664: F, t100638: F, t100641: F, t100644: F, t100646: F, t100651: F, t100656: F, t1649: F, t1877: F, t22959: F, t23295: F, t25013: F, t25354: F, t25372: F, t25892: F, t25921: F, t28771: F, t6670: F, t81483: F, t86736: F, t97972: F, t99064: F) -> F {
    let t100659 = t25927 * t98030;
    let t100664 = t23788 * t98011;
    let t100669 = t1081 * t5664;
    let t100674 = -F::new(3.0) * t25013 * t100638 + F::new(3.0) * t25013 * t100641 + t25372 * t100644 - t1877 * t6670 * t100646 / F::new(2.0) + F::new(6.0) * t25013 * t100651 - F::new(3.0) * t81483 * t28771 - F::new(3.0) * t22959 * t100656 + F::new(2.0) * t25372 * t100659 - F::new(3.0) * t86736 * t25921 - F::new(3.0) / F::new(2.0) * t22959 * t100664 + t1877 * t25354 * t1649 + t1877 * t23295 * t100669 - t97972 + F::new(6.0) * t99064 * t25892;
    t100674
}
