//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2033/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2033<F: Float>(t225: F, t29290: F, t29293: F, t1386: F, t16022: F, t16460: F, t20026: F, t2092: F, t24082: F, t26990: F, t27062: F, t5215: F, t56434: F, t56596: F, t6461: F, t7194: F, t7925: F, t7937: F, t97626: F, t97705: F) -> F {
    let t102917 = t29290 * t225;
    let t102922 = t29293 * t225;
    let t102936 = -F::cast_from(2.0_f64) * t102917 * t1386 - t56596 * t2092 - t56434 * t2092 - t102922 * t1386 + F::cast_from(4.0_f64) * t16022 * t7925 - F::cast_from(2.0_f64) * t16460 * t7937 + F::cast_from(0.3289868133696452873e-1_f64) * t97705 - t24082 * t6461 + F::cast_from(4.0_f64) * t5215 * t27062 + F::cast_from(2.0_f64) * t7194 * t20026 - F::cast_from(12.0_f64) * t97626 * t26990;
    t102936
}
