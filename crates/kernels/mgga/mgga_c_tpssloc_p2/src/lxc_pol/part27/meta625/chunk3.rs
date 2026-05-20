//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2110/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2110<F: Float>(t112: F, t26509: F, t16535: F, t7467: F, t26135: F, t3938: F, t12816: F, t191: F, t192: F, t2020: F, t26161: F, t26162: F, t56404: F) -> (F, F, F, F, F) {
    let t86656 = t26509 * t112;
    let t86660 = F::new(27.0) * t16535 * t7467;
    let t86668 = F::new(27.0) * t3938 * t26135;
    let t86672 = t12816 * t191 * t192;
    let t86673 = t86672 * t2020;
    let t86676 = F::new(4.0) * t26161 * t26162 * t56404;
    (t86656, t86660, t86668, t86673, t86676)
}
