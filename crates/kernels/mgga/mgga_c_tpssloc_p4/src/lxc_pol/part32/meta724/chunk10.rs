//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2328/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2328<F: Float>(t104280: F, t2132: F, t24746: F, t1714: F, t18221: F, t18225: F, t18237: F, t18940: F, t2121: F, t2136: F, t24650: F, t29562: F, t29594: F, t3448: F, t475: F, t6729: F, t68: F, t7321: F, t7326: F, t7328: F, t7573: F, t95340: F, t95346: F, t95387: F, t95515: F, t95517: F, t95520: F) -> F {
    let t104337 = t2132 * t104280 * t24746;
    let t104351 = -t95515 - t2121 * t3448 * t18237 / F::new(144.0) - t2121 * t3448 * t18225 / F::new(72.0) - t2121 * t3448 * t18221 / F::new(48.0) + F::cast_from(0.10093189023535097714e-3_f64) * t7326 * t7328 * t18940 * t68 * t475 - F::cast_from(0.10093189023535097714e-3_f64) * t24650 * t29594 - F::cast_from(0.10093189023535097714e-3_f64) * t104337 - F::cast_from(0.72670960969452703541e-2_f64) * t29562 * t6729 * t2136 + t95517 + t95520 / F::new(648.0) - F::cast_from(0.40372756094140390856e-3_f64) * t95387 * t95340 + F::cast_from(0.20186378047070195428e-3_f64) * t95387 * t95346 + F::cast_from(0.20186378047070195428e-3_f64) * t2132 * t7573 * t1714 * t7321;
    t104351
}
