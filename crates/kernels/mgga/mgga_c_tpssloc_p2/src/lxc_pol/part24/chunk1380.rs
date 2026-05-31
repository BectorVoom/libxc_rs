//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1380/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1380<F: Float>(t10955: F, t1940: F, t354: F, t10459: F, t6765: F, t10870: F, t10489: F, t1046: F, t10501: F, t10915: F, t10919: F, t23529: F, t23544: F, t3043: F, t3064: F, t3098: F, t82843: F, t82848: F, t82851: F, t82859: F, t82861: F, t82863: F) -> F {
    let t82868 = t354 * t1940 * t10955;
    let t82871 = t6765 * t10459;
    let t82875 = t6765 * t10870;
    let t82877 = t6765 * t10489;
    let t82879 = t82843 / F::cast_from(1152.0_f64) - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t23529 * t3064 + t82848 * t3043 / F::cast_from(96.0_f64) - t82851 / F::cast_from(2304.0_f64) - t23544 * t3098 / F::cast_from(384.0_f64) - t6765 * t10915 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t6765 * t10919 + t82859 / F::cast_from(384.0_f64) - t82861 / F::cast_from(768.0_f64) - t82863 / F::cast_from(108.0_f64) + t23529 * t3098 / F::cast_from(72.0_f64) + F::cast_from(19.0_f64) / F::cast_from(432.0_f64) * t82868 * t1046 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t82871 - F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t6765 * t10501 - t82875 / F::cast_from(3456.0_f64) - t82877 / F::cast_from(576.0_f64);
    t82879
}
