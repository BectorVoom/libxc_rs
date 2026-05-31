//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2206/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2206<F: Float>(t25580: F, t3053: F, t23529: F, t4571: F, t13961: F, t6755: F, t14202: F, t6765: F, t13950: F, t14215: F, t14491: F, t1622: F, t23454: F, t3064: F, t7578: F, t82914: F, t82941: F, t82944: F, t83016: F, t83038: F) -> F {
    let t88305 = t25580 * t3053 / F::cast_from(1728.0_f64);
    let t88307 = t23529 * t4571 / F::cast_from(324.0_f64);
    let t88320 = t6755 * t13961 / F::cast_from(1152.0_f64);
    let t88321 = t6765 * t14202;
    let t88324 = t6765 * t13950 / F::cast_from(1728.0_f64);
    let t88327 = t88305 - t88307 - F::cast_from(0.72670960969452703541e-2_f64) * t23454 * t7578 - t82914 / F::cast_from(3456.0_f64) + F::cast_from(0.20186378047070195428e-3_f64) * t82941 - F::cast_from(0.16149102437656156342e-2_f64) * t82944 + t6755 * t14491 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t25580 * t3064 + t83016 * t14215 / F::cast_from(576.0_f64) + t88320 - t88321 / F::cast_from(10368.0_f64) + t88324 - t83038 * t1622 / F::cast_from(216.0_f64);
    t88327
}
