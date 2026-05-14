//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1017/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1017<F: Float>(t13998: F, t973: F, t13542: F, t977: F, t10388: F, t10424: F, t10480: F, t10876: F, t10898: F, t10949: F, t13959: F, t13963: F, t13966: F, t13972: F, t13977: F, t13982: F, t13987: F, t13991: F, t13995: F, t1618: F, t3073: F, t3109: F, t3130: F, t4596: F, t4652: F) -> (F,) {
    let t14000 = t973 * t13998 / 324.0;
    let t14001 = t977 * t13542;
    let t14004 = -t10898 * t1618 / 288.0 - t3109 * t4652 / 288.0 + t13959 + t13963 - t13966 / 13824.0 + 11.0 / 324.0 * t10388 - t13972 + t10949 * t4596 / 768.0 + t3130 * t13977 / 768.0 + t3130 * t13982 / 1536.0 + t10480 * t13987 / 512.0 - t10876 * t13991 / 512.0 + t10424 / 3456.0 + t13995 * t3073 / 2304.0 + t14000 - t973 * t14001 / 72.0;
    (t14004,)
}
