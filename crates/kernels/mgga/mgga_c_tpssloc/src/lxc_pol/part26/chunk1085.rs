//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1085/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1085<F: Float>(t24: F, t39061: F, t111: F, t9346: F, t1307: F, t3914: F, t12442: F, t225: F, t12036: F, t12016: F, t12440: F, t3850: F, t12167: F, t562: F, t12019: F, t566: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t39063 = t24 / t39061;
    let t39235 = t9346 * t111;
    let t39367 = t1307 * t3914;
    let t39910 = t12442 * t225;
    let t39913 = t12036 * t225;
    let t39916 = t12016 * t225;
    let t39919 = t12440 * t225;
    let t40197 = t1307 * t3850;
    let t40475 = t562 * t12167;
    let t40590 = 1.0 / t12019 / t566;
    (t39063, t39235, t39367, t39910, t39913, t39916, t39919, t40197, t40475, t40590)
}
