//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 339/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk339<F: Float>(t880: F, t906: F, t886: F, t897: F, t902: F, t910: F) -> (F, F, F) {
    let t945 = F::new(0.301925e0) * t880;
    let t948 = F::new(0.82785e-1) * t906;
    let t950 = F::new(0.258925e1) * t897 - t945 - F::new(0.301925e0) * t886 + F::new(0.16504875e0) * t902 - t948 - F::new(0.82785e-1) * t910;
    (t945, t948, t950)
}
