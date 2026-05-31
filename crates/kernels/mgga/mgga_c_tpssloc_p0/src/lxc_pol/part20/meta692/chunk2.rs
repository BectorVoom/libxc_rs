//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2637/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2637<F: Float>(t15816: F, t225: F, t11608: F, t11613: F, t11925: F, t11928: F, t1235: F, t1252: F, t14980: F, t15425: F, t15787: F, t15797: F, t15803: F, t3481: F, t3487: F, t3593: F, t3600: F, t3631: F, t466: F, t4945: F, t498: F, t5052: F, t5055: F, t5060: F, t5089: F, t53529: F) -> F {
    let t53703 = t15816 * t225;
    let t53729 = F::cast_from(3.0_f64) * t1235 * t15425 * t498 + F::cast_from(3.0_f64) * t3481 * t498 * t5052 + t466 * t498 * t53529 - F::cast_from(6.0_f64) * t11608 * t4945 - F::cast_from(6.0_f64) * t11608 * t5055 + F::cast_from(12.0_f64) * t11613 * t5060 - F::cast_from(3.0_f64) * t11925 * t5089 + F::cast_from(6.0_f64) * t11928 * t5060 - F::cast_from(6.0_f64) * t1252 * t53703 + F::cast_from(6.0_f64) * t14980 * t3600 - F::cast_from(3.0_f64) * t14980 * t3631 - F::cast_from(3.0_f64) * t15787 * t3487 + F::cast_from(6.0_f64) * t15797 * t3600 + F::cast_from(6.0_f64) * t15803 * t3593;
    t53729
}
