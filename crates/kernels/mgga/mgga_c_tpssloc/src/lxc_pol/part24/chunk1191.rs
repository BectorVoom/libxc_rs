//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1191/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1191<F: Float>(t1933: F, t607: F, t1937: F, t1000: F, t1025: F, t23414: F, t23419: F, t23422: F, t23425: F, t23433: F, t23437: F, t3073: F, t3098: F, t3123: F, t3143: F, t3148: F, t6717: F, t6755: F, t6765: F) -> F {
    let t23442 = t1933 * t607;
    let t23443 = t23442 * t1937;
    let t23445 = F::cast_from(0.10093189023535097714e-3_f64) * t23414 * t1937 + t23419 * t3073 / F::new(1152.0) - t23422 * t1000 / F::new(54.0) + t23425 / F::new(432.0) + t6717 * t3143 / F::new(288.0) + t6717 * t3148 / F::new(216.0) + t6755 * t3123 / F::new(1536.0) + t23433 * t1025 / F::new(768.0) - t23437 * t1025 / F::new(144.0) - t6765 * t3098 / F::new(1152.0) + F::cast_from(0.20186378047070195428e-3_f64) * t23443;
    t23445
}
