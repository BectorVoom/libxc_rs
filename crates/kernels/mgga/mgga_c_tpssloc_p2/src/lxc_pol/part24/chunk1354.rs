//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1354/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1354<F: Float>(t10103: F, t10116: F, t1902: F, t1911: F, t22975: F, t23150: F, t23278: F, t259: F, t2597: F, t2718: F, t2720: F, t2743: F, t6627: F, t6632: F, t798: F, t82255: F, t82259: F, t82266: F, t855: F, t9584: F, t9593: F) -> F {
    let t82279 = t9584 * t1902 * t259 - F::new(3.0) * t23278 * t2743 - F::cast_from(0.49348022005446793095e-1_f64) * t82255 + F::new(6.0) * t23278 * t2720 + F::cast_from(0.19190897446562641759e0_f64) * t82259 + F::new(6.0) * t6627 * t10116 + F::cast_from(0.14804406601634037928e0_f64) * t82266 + F::new(2.0) * t855 * t2718 * t1911 * t10103 + F::new(6.0) * t2597 * t22975 + F::new(12.0) * t9593 * t6632 + F::new(3.0) * t798 * t23150 * t259;
    t82279
}
