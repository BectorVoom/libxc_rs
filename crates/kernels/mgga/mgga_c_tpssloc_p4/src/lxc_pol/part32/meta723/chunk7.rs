//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2313/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2313<F: Float>(t24574: F, t29777: F, t29678: F, t7359: F, t29759: F, t1244: F, t1246: F, t15245: F, t1734: F, t19120: F, t19169: F, t2121: F, t2147: F, t24776: F, t24858: F, t27406: F, t27546: F, t27574: F, t27721: F, t29711: F, t3624: F, t462: F, t5079: F, t5971: F, t7283: F, t7373: F, t7375: F, t7376: F, t95714: F, t95722: F) -> F {
    let t103927 = t24574 * t29777;
    let t103939 = t29678 * t7359;
    let t103943 = t24574 * t29759;
    let t103949 = F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27574 + F::cast_from(0.36554090374405031923e-2_f64) * t7283 * t24776 * t24858 * t5971 + F::cast_from(0.12184696791468343974e-2_f64) * t103927 + F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t7375 * t19169 * t7376 + F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t462 * t2147 * t19120 - t95714 - t3624 * t29711 * t5079 + F::cast_from(0.26806332941230356743e-1_f64) * t103939 - F::cast_from(2.0_f64) * t15245 * t27546 - F::cast_from(0.91385225936012579807e-3_f64) * t103943 + F::cast_from(2.0_f64) * t1244 * t27721 * t1734 * t1246 - t95722;
    t103949
}
