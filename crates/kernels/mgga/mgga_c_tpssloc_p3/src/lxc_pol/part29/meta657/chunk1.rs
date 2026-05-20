//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2184/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2184<F: Float>(t1845: F, t3914: F, t26161: F, t26162: F, t24994: F, t6875: F, t24996: F, t24995: F, t34475: F, t5308: F, t1983: F, t26503: F, t6999: F) -> (F, F, F, F) {
    let t90437 = t1845 * t3914;
    let t90440 = F::new(2.0) * t26161 * t26162 * t90437;
    let t90442 = t6875 * t24994;
    let t90444 = F::new(12.0) * t90442 * t24996;
    let t90447 = F::new(12.0) * t24995 * t34475 * t5308;
    let t90450 = F::new(2.0) * t1983 * t26503 * t6999;
    (t90440, t90444, t90447, t90450)
}
