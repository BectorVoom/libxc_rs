//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3005/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3005<F: Float>(t13995: F, t14501: F, t1020: F, t1021: F, t10214: F, t10403: F, t10408: F, t1041: F, t14164: F, t14211: F, t1539: F, t17701: F, t17732: F, t18014: F, t248: F, t2979: F, t3040: F, t3070: F, t3071: F, t3120: F, t360: F, t42388: F, t42546: F, t42861: F, t43343: F, t4338: F, t4582: F, t4650: F, t48612: F, t50337: F, t5875: F, t59706: F, t59711: F, t59719: F, t61910: F, t62757: F, t62766: F, t62778: F, t973: F) -> F {
    let t62780 = t13995 * t14501;
    let t62803 = t1041 * t4582 * t14164 * t61910 / F::cast_from(768.0_f64) + t1020 * t248 * t1021 * t62757 * t360 / F::cast_from(3072.0_f64) + t43343 * t5875 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(972.0_f64) * t62766 + t973 * t2979 * t59719 / F::cast_from(108.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t973 * t10214 * t59706 + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t973 * t42861 * t59711 - t62778 / F::cast_from(384.0_f64) + t62780 / F::cast_from(1728.0_f64) + F::cast_from(19.0_f64) / F::cast_from(1296.0_f64) * t50337 + t10403 * t3071 * t17732 * t18014 / F::cast_from(576.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t3070 * t10408 * t4650 * t4338 + t10403 * t3071 * t14211 * t1539 * t3120 / F::cast_from(1152.0_f64) + t42388 * t3071 * t48612 * t1539 * t3040 / F::cast_from(384.0_f64) - t42546 * t17701 / F::cast_from(2304.0_f64);
    t62803
}
