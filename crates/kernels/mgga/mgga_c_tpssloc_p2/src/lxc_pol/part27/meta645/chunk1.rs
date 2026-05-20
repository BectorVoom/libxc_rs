//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2208/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2208<F: Float>(t23512: F, t25486: F, t23519: F, t25492: F, t1607: F, t23515: F, t23521: F, t23529: F, t4636: F, t6747: F, t82911: F, t82951: F, t82953: F, t83092: F, t88335: F, t88336: F, t88339: F, t88341: F, t88342: F, t88348: F) -> F {
    let t88351 = t23512 * t25486;
    let t88354 = t23519 * t25492;
    let t88358 = -t23529 * t4636 / F::new(216.0) + F::new(11.0) / F::new(324.0) * t83092 * t1607 - t88335 - t88336 / F::new(1296.0) + t88339 - t88341 + F::cast_from(0.20186378047070195428e-3_f64) * t88342 * t23515 - F::cast_from(0.10093189023535097714e-3_f64) * t88342 * t23521 - F::cast_from(0.10093189023535097714e-3_f64) * t82951 + F::cast_from(0.16149102437656156342e-2_f64) * t88348 * t6747 - F::cast_from(0.40372756094140390856e-3_f64) * t82911 * t88351 + F::cast_from(0.20186378047070195428e-3_f64) * t82911 * t88354 + t82953 / F::new(1152.0);
    t88358
}
