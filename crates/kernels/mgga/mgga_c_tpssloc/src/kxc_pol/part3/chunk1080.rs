//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1080/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1080<F: Float>(t13985: F, t4593: F, t4582: F, t3132: F, t3069: F, t4669: F, t10231: F, t4338: F, t973: F, t13542: F, t977: F, t10388: F, t10424: F, t10480: F, t10876: F, t10898: F, t10949: F, t13959: F, t13963: F, t13966: F, t13972: F, t13977: F, t13982: F, t1618: F, t3073: F, t3109: F, t3130: F, t4596: F, t4652: F) -> F {
    let t13986 = t4593 * t13985;
    let t13987 = t4582 * t13986;
    let t13990 = t4593 * t3132;
    let t13991 = t4582 * t13990;
    let t13995 = t4669 * t3069;
    let t13998 = t10231 * t4338;
    let t14000 = t973 * t13998 / F::new(324.0);
    let t14001 = t977 * t13542;
    let t14004 = -t10898 * t1618 / F::new(288.0) - t3109 * t4652 / F::new(288.0) + t13959 + t13963 - t13966 / F::new(13824.0) + F::new(11.0) / F::new(324.0) * t10388 - t13972 + t10949 * t4596 / F::new(768.0) + t3130 * t13977 / F::new(768.0) + t3130 * t13982 / F::new(1536.0) + t10480 * t13987 / F::new(512.0) - t10876 * t13991 / F::new(512.0) + t10424 / F::new(3456.0) + t13995 * t3073 / F::new(2304.0) + t14000 - t973 * t14001 / F::new(72.0);
    t14004
}
