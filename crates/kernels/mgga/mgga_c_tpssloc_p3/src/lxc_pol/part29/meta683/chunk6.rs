//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2320/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2320<F: Float>(t3540: F, t8049: F, t2132: F, t2136: F, t3966: F, t24716: F, t4997: F, t15459: F, t15463: F, t15470: F, t15710: F, t24706: F, t24736: F, t24741: F, t27674: F, t3562: F, t5030: F, t8031: F, t86293: F, t86299: F) -> F {
    let t95520 = t8049 * t3540;
    let t95540 = F::cast_from(0.20186378047070195428e-3_f64) * t2132 * t3966 * t2136;
    let t95542 = t24716 * t4997 / F::cast_from(1152.0_f64);
    let t95543 = t95520 / F::cast_from(1296.0_f64) - t27674 * t3562 / F::cast_from(81.0_f64) - t24741 * t15459 / F::cast_from(2304.0_f64) - t24741 * t15470 / F::cast_from(1152.0_f64) - t24741 * t15463 / F::cast_from(1152.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t8031 * t24706 - F::cast_from(0.20186378047070195428e-3_f64) * t86293 - t24741 * t15710 / F::cast_from(576.0_f64) - t24736 * t5030 / F::cast_from(1152.0_f64) + t86299 / F::cast_from(1152.0_f64) - t95540 + t95542;
    t95543
}
