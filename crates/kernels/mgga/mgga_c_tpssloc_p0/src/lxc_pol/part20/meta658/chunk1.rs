//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2441/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2441<F: Float>(t49922: F, t10408: F, t10428: F, t10919: F, t14152: F, t14508: F, t1618: F, t2771: F, t2960: F, t3070: F, t42573: F, t42658: F, t43103: F, t43110: F, t4600: F, t4644: F, t4650: F, t47746: F, t49892: F, t49894: F, t49897: F, t49907: F, t973: F, t977: F) -> F {
    let t49923 = t49922 / F::new(2304.0);
    let t49924 = -F::new(5.0) / F::new(648.0) * t49892 - t49894 / F::new(768.0) - t49897 / F::new(768.0) + t42573 * t4600 / F::new(96.0) + t14508 * t10428 / F::new(512.0) + F::new(5.0) / F::new(4608.0) * t4644 * t10919 + t49907 - t2960 * t14152 / F::new(6.0) - t973 * t977 * t47746 / F::new(12.0) + F::new(5.0) / F::new(4608.0) * t3070 * t10408 * t4650 * t2771 + F::new(7.0) / F::new(1944.0) * t43103 + t43110 / F::new(216.0) - F::new(209.0) / F::new(2592.0) * t42658 * t1618 - t49923;
    t49924
}
