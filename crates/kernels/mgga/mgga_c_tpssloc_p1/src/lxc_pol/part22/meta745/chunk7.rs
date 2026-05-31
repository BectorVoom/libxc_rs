//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2480/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2480<F: Float>(t10403: F, t10422: F, t21525: F, t1023: F, t10408: F, t1041: F, t10876: F, t14508: F, t1539: F, t17670: F, t17714: F, t17732: F, t17890: F, t17960: F, t21118: F, t21398: F, t21512: F, t3048: F, t3070: F, t3071: F, t42565: F, t4582: F, t4644: F, t47779: F, t62210: F, t62234: F, t70330: F) -> F {
    let t70535 = t10403 * t10422 * t21525;
    let t70539 = t42565 * t21398 / F::cast_from(96.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t62210 - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t10876 * t4582 * t17670 * t17732 + t3070 * t3071 * t17960 * t1539 / F::cast_from(1536.0_f64) - t62234 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t1041 * t4582 * t47779 * t70330 - F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t3070 * t10408 * t21118 * t1023 + t14508 * t17714 / F::cast_from(512.0_f64) - F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t3048 * t21512 + t70535 / F::cast_from(1152.0_f64) + t4644 * t17890 / F::cast_from(1536.0_f64);
    t70539
}
