//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1312/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1312<F: Float>(t10277: F, t976: F, t1021: F, t10263: F, t10403: F, t1041: F, t10493: F, t248: F, t2776: F, t3039: F, t3048: F, t3070: F, t3071: F, t3121: F, t3132: F, t3146: F, t3151: F, t3153: F, t360: F, t39097: F, t39103: F, t42374: F, t42412: F, t42422: F, t42428: F, t42432: F, t42436: F, t4582: F, t4588: F, t973: F, t974: F) -> F {
    let t42444 = t976 * t10277;
    let t42459 = t42412 / F::new(576.0) - t3070 * t3071 * t3121 * t2776 / F::new(384.0) - t10403 * t3071 * t3132 * t2776 / F::new(192.0) - t3039 * t248 * t1021 * t42422 * t360 / F::new(1024.0) + F::new(19.0) / F::new(216.0) * t42428 - t42432 / F::new(3456.0) + t42436 / F::new(288.0) - t3048 * t10493 / F::new(36.0) + F::new(5.0) / F::new(3456.0) * t1041 * t4582 * t4588 * t42374 - t973 * t974 * t42444 * t39097 / F::new(12.0) - t973 * t974 * t3151 * t39103 / F::new(48.0) + t973 * t974 * t3146 * t39103 / F::new(72.0) - F::new(11.0) / F::new(27.0) * t10263 * t3153;
    t42459
}
