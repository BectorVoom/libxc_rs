//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2662/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2662<F: Float>(t20512: F, t40021: F, t1351: F, t6347: F, t16288: F, t6422: F, t1363: F, t1367: F, t16225: F, t16233: F, t16305: F, t16311: F, t1827: F, t19855: F, t19904: F, t20473: F, t5246: F, t5289: F, t5310: F, t53985: F, t53998: F, t56693: F, t56710: F, t56738: F, t56924: F, t57342: F, t74355: F, t820: F) -> F {
    let t74360 = t40021 * t20512;
    let t74366 = t6347 * t1351;
    let t74376 = t16288 * t6422;
    let t74386 = -t1363 * t1367 * t820 * t74355 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t74360 - t5246 * t16305 * t20473 * t16225 / F::cast_from(128.0_f64) - t5246 * t16305 * t16311 * t74366 / F::cast_from(128.0_f64) + F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t16233 * t16305 * t57342 * t16225 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t56693 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t74376 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t56710 - t53985 + t53998 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t56738 - t56924 * t1827 / F::cast_from(1024.0_f64) - t19855 * t5289 / F::cast_from(1024.0_f64) + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t19904 * t5310;
    t74386
}
