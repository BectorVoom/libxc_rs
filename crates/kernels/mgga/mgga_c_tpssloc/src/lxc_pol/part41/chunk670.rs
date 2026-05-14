//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 670/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk670<F: Float>(t4608: F, t974: F, t1041: F, t1607: F, t1622: F, t2960: F, t3039: F, t3048: F, t3054: F, t3070: F, t3084: F, t3092: F, t3130: F, t4562: F, t4565: F, t4572: F, t4575: F, t4579: F, t4585: F, t4590: F, t4596: F, t4600: F, t4604: F, t973: F) -> (F,) {
    let t4609 = t974 * t4608;
    let t4613 = t3054 / 6912.0 - t973 * t4562 / 144.0 + t973 * t4565 / 216.0 - t3048 * t1622 / 864.0 + t4572 / 6912.0 + t3070 * t4575 / 4608.0 + t3070 * t4579 / 4608.0 - t1041 * t4585 / 2304.0 + 5.0 / 13824.0 * t1041 * t4590 + t3130 * t4596 / 1536.0 - t3039 * t4600 / 3072.0 + t4604 / 864.0 - t2960 * t1607 / 108.0 + t973 * t4609 / 288.0 - t3084 - t3092 / 864.0;
    (t4613,)
}
