//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2280/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2280<F: Float>(t17667: F, t23537: F, t1622: F, t17925: F, t17962: F, t23529: F, t5861: F, t5875: F, t5880: F, t6755: F, t82848: F, t82851: F, t82956: F, t83043: F, t83061: F, t83215: F, t88249: F, t88584: F) -> F {
    let t99483 = t23537 * t17667;
    let t99492 = -t83215 * t17925 / F::cast_from(1152.0_f64) + t6755 * t17962 / F::cast_from(1536.0_f64) + t83043 * t5875 / F::cast_from(768.0_f64) - t83061 * t5880 / F::cast_from(1536.0_f64) - t82956 * t5875 / F::cast_from(144.0_f64) + t99483 / F::cast_from(1152.0_f64) + t82848 * t5880 / F::cast_from(288.0_f64) - t82851 / F::cast_from(6912.0_f64) - t88584 * t1622 / F::cast_from(216.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t23529 * t5861 + t88249;
    t99492
}
