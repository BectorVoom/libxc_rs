//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1087/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1087<F: Float>(t6699: F, t986: F, t3206: F, t6705: F, t6704: F, t1922: F, t3016: F, t2261: F, t337: F, t1887: F) -> (F, F, F, F, F, F) {
    let t23310 = t986 * t6699;
    let t23313 = t6705 * t3206;
    let t23314 = t6704 * t23313;
    let t23317 = t3016 * t1922;
    let t23322 = t2261 * t337;
    let t23323 = t23322 * t1887;
    (t23310, t23313, t23314, t23317, t23322, t23323)
}
