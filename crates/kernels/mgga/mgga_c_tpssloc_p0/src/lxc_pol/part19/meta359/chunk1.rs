//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1305/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1305<F: Float>(t10214: F, t10378: F, t1041: F, t10463: F, t10863: F, t10879: F, t248: F, t2960: F, t2979: F, t3062: F, t3098: F, t3117: F, t39097: F, t41644: F, t41693: F, t41697: F, t41701: F, t41705: F, t42303: F, t42309: F, t42322: F, t42324: F, t42334: F, t973: F, t974: F, t977: F) -> F {
    let t42337 = F::new(2.0) / F::new(9.0) * t2960 * t10378 + F::new(7.0) / F::new(108.0) * t973 * t10214 * t41693 + F::new(5.0) / F::new(4608.0) * t1041 * t248 * t3062 * t41701 + F::new(19.0) / F::new(324.0) * t42303 + t10863 * t3098 / F::new(36.0) + F::new(35.0) / F::new(972.0) * t973 * t974 * t42309 * t39097 - t973 * t977 * t41644 / F::new(36.0) + t973 * t2979 * t41705 / F::new(54.0) + t42322 / F::new(1728.0) + F::new(5.0) / F::new(1728.0) * t42324 + t3117 * t10463 / F::new(1152.0) + F::new(5.0) / F::new(384.0) * t1041 * t248 * t3062 * t41697 - t42334 * t10879 / F::new(128.0);
    t42337
}
