//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1124/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1124<F: Float>(t13985: F, t4593: F, t4582: F, t3132: F, t3069: F, t4669: F, t10231: F, t4338: F, t973: F, t13542: F, t977: F, t10388: F, t10424: F, t10480: F, t10876: F, t10898: F, t10949: F, t13959: F, t13963: F, t13966: F, t13972: F, t13977: F, t13982: F, t1618: F, t3073: F, t3109: F, t3130: F, t4596: F, t4652: F) -> F {
    let t13986 = t4593 * t13985;
    let t13987 = t4582 * t13986;
    let t13990 = t4593 * t3132;
    let t13991 = t4582 * t13990;
    let t13995 = t4669 * t3069;
    let t13998 = t10231 * t4338;
    let t14000 = t973 * t13998 / F::cast_from(324.0_f64);
    let t14001 = t977 * t13542;
    let t14004 = -t10898 * t1618 / F::cast_from(288.0_f64) - t3109 * t4652 / F::cast_from(288.0_f64) + t13959 + t13963 - t13966 / F::cast_from(13824.0_f64) + F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t10388 - t13972 + t10949 * t4596 / F::cast_from(768.0_f64) + t3130 * t13977 / F::cast_from(768.0_f64) + t3130 * t13982 / F::cast_from(1536.0_f64) + t10480 * t13987 / F::cast_from(512.0_f64) - t10876 * t13991 / F::cast_from(512.0_f64) + t10424 / F::cast_from(3456.0_f64) + t13995 * t3073 / F::cast_from(2304.0_f64) + t14000 - t973 * t14001 / F::cast_from(72.0_f64);
    t14004
}
