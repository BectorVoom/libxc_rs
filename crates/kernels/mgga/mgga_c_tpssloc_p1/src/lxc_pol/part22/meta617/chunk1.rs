//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2148/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2148<F: Float>(t1174: F, t44571: F, t4724: F, t11778: F, t43791: F, t1227: F, t49850: F, t4988: F, t15568: F, t3604: F, t10401: F, t15567: F) -> (F, F, F, F, F) {
    let t52599 = t1174 * t44571 * t4724;
    let t52600 = t52599 / F::new(324.0);
    let t52601 = t11778 * t43791;
    let t52609 = t1227 * t49850 * t4988;
    let t52610 = F::new(5.0) / F::new(20736.0) * t52609;
    let t52615 = t3604 * t15568;
    let t52627 = t15567 * t10401;
    (t52600, t52601, t52610, t52615, t52627)
}
