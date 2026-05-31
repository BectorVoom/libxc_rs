//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 678/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk678<F: Float>(t135: F, t1606: F, t973: F, t3966: F, t998: F, t974: F, t1041: F, t1607: F, t1622: F, t2960: F, t3039: F, t3048: F, t3054: F, t3070: F, t3084: F, t3092: F, t3130: F, t4562: F, t4565: F, t4572: F, t4575: F, t4579: F, t4585: F, t4590: F, t4596: F, t4600: F) -> (F, F, F, F) {
    let t4603 = t135 * t1606;
    let t4604 = t973 * t4603;
    let t4608 = t998 * t3966;
    let t4609 = t974 * t4608;
    let t4613 = t3054 / F::cast_from(6912.0_f64) - t973 * t4562 / F::cast_from(144.0_f64) + t973 * t4565 / F::cast_from(216.0_f64) - t3048 * t1622 / F::cast_from(864.0_f64) + t4572 / F::cast_from(6912.0_f64) + t3070 * t4575 / F::cast_from(4608.0_f64) + t3070 * t4579 / F::cast_from(4608.0_f64) - t1041 * t4585 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1041 * t4590 + t3130 * t4596 / F::cast_from(1536.0_f64) - t3039 * t4600 / F::cast_from(3072.0_f64) + t4604 / F::cast_from(864.0_f64) - t2960 * t1607 / F::cast_from(108.0_f64) + t973 * t4609 / F::cast_from(288.0_f64) - t3084 - t3092 / F::cast_from(864.0_f64);
    (t4603, t4604, t4608, t4613)
}
