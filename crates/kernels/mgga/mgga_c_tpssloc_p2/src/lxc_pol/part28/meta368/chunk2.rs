//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1386/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1386<F: Float>(t14032: F, t3071: F, t1616: F, t2771: F, t10408: F, t1539: F, t3121: F, t3048: F, t4571: F, t10390: F, t10891: F, t10904: F, t10937: F, t10957: F, t14006: F, t14009: F, t14012: F, t14015: F, t14018: F, t14027: F, t1622: F, t3070: F, t3098: F, t4575: F, t4596: F, t4600: F, t4644: F, t973: F) -> F {
    let t14033 = t3071 * t14032;
    let t14036 = t1616 * t2771;
    let t14037 = t10408 * t14036;
    let t14040 = t1539 * t3121;
    let t14041 = t3071 * t14040;
    let t14049 = t3048 * t4571 / F::cast_from(648.0_f64);
    let t14050 = -t973 * t14006 / F::cast_from(144.0_f64) - t973 * t14009 / F::cast_from(36.0_f64) + t973 * t14012 / F::cast_from(108.0_f64) + t973 * t14015 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t973 * t14018 - t10904 * t4596 / F::cast_from(144.0_f64) + t10891 * t4600 / F::cast_from(288.0_f64) + t14027 + t10390 * t4575 / F::cast_from(2304.0_f64) - t10937 * t4575 / F::cast_from(432.0_f64) + t3070 * t14033 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3070 * t14037 + t3070 * t14041 / F::cast_from(4608.0_f64) - t4644 * t3098 / F::cast_from(2304.0_f64) + F::cast_from(19.0_f64) / F::cast_from(2592.0_f64) * t10957 * t1622 - t14049;
    t14050
}
