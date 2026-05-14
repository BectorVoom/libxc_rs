//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1213/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1213<F: Float>(t24785: F, t24826: F, t7368: F, t85660: F, t24574: F, t24781: F, t24789: F, t85639: F, t11553: F, t2121: F, t2148: F, t24765: F, t27561: F, t7327: F, t52537: F, t7376: F) -> (F, F, F, F, F, F, F, F) {
    let t85984 = t24826 * t24785;
    let t85986 = t85660 * t7368;
    let t85988 = t24574 * t24781;
    let t85996 = t85639 * t24789;
    let t86000 = 0.30461741978670859935e-2 * t2121 * t11553 * t2148;
    let t86001 = t24574 * t24765;
    let t86015 = t7327 * t27561;
    let t86016 = t52537 * t7376;
    (t85984, t85986, t85988, t85996, t86000, t86001, t86015, t86016)
}
