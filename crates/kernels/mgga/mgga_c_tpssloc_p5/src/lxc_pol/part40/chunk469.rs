//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 469/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk469<F: Float>(t1020: F, t1038: F, t1041: F, t1607: F, t1612: F, t1618: F, t1622: F, t378: F, t973: F, t997: F) -> F {
    let t1625 = t997 + t973 * t1607 / F::new(288.0) + t1612 * t378 / F::new(3072.0) + t1020 * t1618 / F::new(3072.0) + t1038 + t1041 * t1622 / F::new(4608.0);
    t1625
}
