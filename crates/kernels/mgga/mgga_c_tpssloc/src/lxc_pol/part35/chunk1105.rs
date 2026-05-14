//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1105/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1105<F: Float>(t1888: F, t28427: F, t1909: F, t226: F, t23174: F, t25310: F, t26613: F, t26667: F, t26673: F, t28407: F, t28409: F, t28411: F, t28413: F, t28420: F, t28424: F, t5575: F, t812: F) -> (F,) {
    let t28428 = t1888 * t28427;
    let t28430 = t226 * t28407 - t23174 + t26613 - t812 * t28409 - t812 * t28411 + 2.0 * t812 * t28413 - t26667 + t5575 * t1909 + 0.76763589786250567036e-1 * t25310 + t26673 - 0.16449340668482264365e-1 * t28420 - 0.82246703342411321825e-2 * t28424 + 0.16449340668482264365e-1 * t28428;
    (t28430,)
}
