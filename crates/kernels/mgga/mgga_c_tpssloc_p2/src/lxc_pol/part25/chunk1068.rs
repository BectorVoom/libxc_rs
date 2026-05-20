//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1068/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1068<F: Float>(t113: F, t12823: F, t1983: F, t2040: F, t2075: F, t2096: F, t22574: F, t22607: F, t2312: F, t2314: F, t2320: F, t23958: F, t24008: F, t24026: F, t24028: F, t24167: F, t24169: F, t24176: F, t24428: F, t24433: F, t24442: F, t4034: F, t510: F, t574: F, t650: F, t652: F, t6876: F, t7050: F, t7057: F, t7156: F, t7171: F, t7218: F, t7220: F) -> F {
    let t24446 = F::new(6.0) * t1983 * t23958 - F::new(2.0) * t6876 * t7220 + t24026 * t574 - F::new(2.0) * t1983 * t24028 + t1983 * t24167 + F::new(2.0) * t1983 * t24169 - F::new(2.0) * t650 * t7156 - t2312 * t2075 + F::new(6.0) * t1983 * t24176 + F::new(6.0) * t6876 * t7171 + t22607 * t2096 + F::new(2.0) * t6876 * t7218 - t113 * t24428 - F::new(2.0) * t2320 * t2075 - F::new(6.0) * t22574 * t24433 - F::new(4.0) * t2314 * t7057 - F::new(2.0) * t12823 * t2040 - F::new(4.0) * t4034 * t7050 - F::new(2.0) * t652 * t24442 - t24008 * t510;
    t24446
}
