//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 885/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk885<F: Float>(t17808: F, t2986: F, t10254: F, t5392: F, t135: F, t5844: F, t973: F, t5838: F, t10236: F, t10457: F, t248: F, t5677: F) -> (F, F, F, F, F, F) {
    let t17809 = t2986 * t17808;
    let t17817 = t10254 * t5392;
    let t17826 = t135 * t5844;
    let t17827 = t973 * t17826;
    let t17849 = t135 * t5838;
    let t17850 = t973 * t17849;
    let t17863 = t10236 * t5392;
    let t17884 = t248 * t10457 * t5677;
    (t17809, t17817, t17827, t17850, t17863, t17884)
}
