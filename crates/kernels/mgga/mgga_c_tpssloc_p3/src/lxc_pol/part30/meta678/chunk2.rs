//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2122/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2122<F: Float>(t1458: F, t19534: F, t22461: F, t24999: F, t26103: F, t33085: F, t4072: F, t5493: F, t6517: F, t671: F, t90400: F, t96361: F, t96685: F, t96686: F, t96704: F, t96706: F, t96708: F, t96711: F, t96731: F) -> F {
    let t96732 = F::new(4.0) * t1458 * t90400 + F::new(4.0) * t1458 * t96361 + F::new(2.0) * t19534 * t6517 + F::new(2.0) * t22461 * t5493 + F::new(4.0) * t24999 * t4072 + F::new(2.0) * t26103 * t5493 + F::new(4.0) * t33085 * t4072 + F::new(2.0) * t671 * t96686 + t96685 + t96704 + t96706 + t96708 + t96711 + t96731;
    t96732
}
