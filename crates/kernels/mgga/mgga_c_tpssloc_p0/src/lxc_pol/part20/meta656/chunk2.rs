//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2426/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2426<F: Float>(t3041: F, t607: F, t1023: F, t3120: F, t10390: F, t14501: F, t10422: F, t13761: F, t3070: F, t1020: F, t1021: F, t1031: F, t10413: F, t13941: F, t14093: F, t1539: F, t248: F, t3071: F, t3088: F, t3117: F, t360: F, t378: F, t42514: F, t42518: F, t4342: F, t4347: F, t4616: F, t48670: F, t48674: F, t49588: F) -> (F, F, F) {
    let t49594 = t3041 * t607;
    let t49599 = t1023 * t3120;
    let t49604 = t10390 * t14501;
    let t49607 = t3070 * t10422 * t13761;
    let t49609 = -t10413 * t3071 * t4347 * t3041 / F::cast_from(1536.0_f64) - t42514 / F::cast_from(432.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t42518 + F::cast_from(19.0_f64) / F::cast_from(576.0_f64) * t4616 * t3088 * t378 - t13941 * t1031 * t378 / F::cast_from(192.0_f64) + t48670 / F::cast_from(10368.0_f64) + t48674 / F::cast_from(15552.0_f64) + t3117 * t14093 / F::cast_from(1536.0_f64) + t1020 * t248 * t1021 * t49588 * t360 / F::cast_from(3072.0_f64) + t10413 * t3071 * t4342 * t49594 / F::cast_from(768.0_f64) - t10413 * t3071 * t1539 * t49599 / F::cast_from(1536.0_f64) + t49604 / F::cast_from(1152.0_f64) + t49607 / F::cast_from(1152.0_f64);
    (t49594, t49599, t49609)
}
