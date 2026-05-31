//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2595/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2595<F: Float>(t1227: F, t49850: F, t4988: F, t15568: F, t3604: F, t11697: F, t15473: F, t3577: F, t11698: F, t15740: F, t10401: F, t15567: F) -> (F, F, F, F, F) {
    let t52609 = t1227 * t49850 * t4988;
    let t52610 = F::cast_from(5.0_f64) / F::cast_from(20736.0_f64) * t52609;
    let t52615 = t3604 * t15568;
    let t52619 = t3577 * t11697 * t15473;
    let t52621 = t15740 * t11698;
    let t52627 = t15567 * t10401;
    (t52610, t52615, t52619, t52621, t52627)
}
