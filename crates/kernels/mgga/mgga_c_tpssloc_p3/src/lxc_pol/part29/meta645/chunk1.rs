//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2129/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2129<F: Float>(t23098: F, t7496: F, t87447: F, t6590: F, t6646: F, t25130: F, t81918: F, t81921: F, t81924: F, t81926: F, t81936: F, t87418: F, t87422: F, t87426: F, t87428: F, t87430: F, t87432: F, t87437: F, t87438: F, t87440: F, t87444: F, t87445: F) -> F {
    let t87449 = t87447 * t7496 * t23098;
    let t87451 = t6590 * t6646;
    let t87453 = t87451 * t25130 * t23098;
    let t87455 = F::cast_from(0.16956557559538964158e-1_f64) * t87418 - t87422 / F::cast_from(4.0_f64) - t87426 + F::cast_from(0.84782787797694820792e-2_f64) * t87428 - t87430 / F::cast_from(48.0_f64) - F::cast_from(0.11304371706359309439e-1_f64) * t87432 - F::cast_from(0.6728792682356731809e-4_f64) * t81918 - t81921 + F::cast_from(0.33643963411783659045e-4_f64) * t81924 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t81926 + t87437 - t87438 + F::cast_from(0.84782787797694820794e-2_f64) * t81936 - t87440 + t87444 + F::cast_from(0.10093189023535097714e-3_f64) * t87445 - F::cast_from(0.16956557559538964158e-1_f64) * t87449 + F::cast_from(0.24223653656484234512e-2_f64) * t87453;
    t87455
}
