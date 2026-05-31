//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2344/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2344<F: Float>(t103125: F, t104721: F, t113: F, t1393: F, t1849: F, t19289: F, t19450: F, t20098: F, t2114: F, t2165: F, t2167: F, t27903: F, t29497: F, t33690: F, t4073: F, t96355: F, t96358: F, t96360: F, t96738: F, t96740: F, t96746: F, t96755: F, t96758: F, t96760: F, t96763: F, t96765: F, t96767: F) -> F {
    let t104727 = t96355 - t96358 - t96360 + t29497 * t1393 - t2114 * t19289 - t96738 - t96740 + F::cast_from(2.0_f64) * t27903 * t1849 + t96746 - t96755 - t96758 + t96760 + t2167 * t20098 - t113 * (t103125 + t104721) - t19450 * t2165 + t96763 - t96765 - t96767 - F::cast_from(4.0_f64) * t33690 * t4073;
    t104727
}
