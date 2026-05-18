//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1154/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1154<F: Float>(t24503: F, t67: F, t1864: F, t6509: F, t7255: F, t2109: F, t22489: F, t7245: F, t9239: F, t22550: F, t9231: F, t33: F, t7254: F) -> (F, F, F, F, F, F, F, F) {
    let t24504 = t24503 * t67;
    let t24505 = t24504 * t1864;
    let t24508 = t7255 * t6509;
    let t24511 = t2109 * t22489;
    let t24514 = t9239 * t7245;
    let t24517 = t2109 * t22550;
    let t24520 = t9231 * t7245;
    let t24525 = t33 * t7254;
    (t24504, t24505, t24508, t24511, t24514, t24517, t24520, t24525)
}
