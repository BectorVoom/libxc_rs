//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2122/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2122<F: Float>(t1580: F, t2930: F, t2885: F, t4408: F, t47705: F, t47707: F, t47730: F, t10632: F, t4471: F, t48096: F, t2904: F, t4446: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t48783 = t2930 * t1580;
    let t48789 = t4408 * t2885;
    let t48799 = F::cast_from(0.4566222222222222222e-1_f64) * t47705;
    let t48800 = F::cast_from(0.1522074074074074074e-1_f64) * t47707;
    let t48809 = F::cast_from(0.2283111111111111111e-1_f64) * t47730;
    let t48890 = t4471 * t10632;
    let t48919 = F::cast_from(0.27385555555555555556e0_f64) * t48096;
    let t48924 = F::cast_from(0.39862222222222222223e0_f64) * t47730;
    let t48946 = F::new(8.0) / F::new(9.0) * t47705;
    let t48947 = F::new(8.0) / F::new(27.0) * t47707;
    let t48956 = F::new(4.0) / F::new(9.0) * t47730;
    let t49096 = t4446 * t2904;
    (t48783, t48789, t48799, t48800, t48809, t48890, t48919, t48924, t48946, t48947, t48956, t49096)
}
