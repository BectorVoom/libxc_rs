//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1298/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1298<F: Float>(t111457: F, t111503: F, t111546: F, t111592: F, t110489: F, t110882: F, t110884: F, t110886: F, t110888: F, t111316: F, t111317: F, t111322: F, t1404: F, t1852: F, t20186: F, t2187: F, t3: F, t30263: F, t30466: F, t580: F, t6483: F, t8154: F) -> (F, F) {
    let t111594 = t111457 + t111503 + t111546 + t111592;
    let t111597 = t111594 * t3 * t580 + t1404 * t30466 + F::new(2.0) * t1852 * t30263 + t20186 * t2187 + t6483 * t8154 + t110489 + t110882 + t110884 + t110886 + t110888 + t111316 + F::new(2.0) * t111317 + t111322;
    (t111594, t111597)
}
