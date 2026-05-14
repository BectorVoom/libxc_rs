//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1172/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1172<F: Float>(t6552: F, t6555: F, t82124: F, t23035: F, t23237: F, t23241: F, t23219: F, t6547: F, t23265: F, t23030: F, t23208: F, t82120: F, t82123: F, t82126: F, t82129: F, t82131: F, t82135: F) -> (F,) {
    let t82138 = t6552 * t82124 * t6555;
    let t82141 = t23035 * t23237 * t23241;
    let t82143 = t6547 * t23219;
    let t82145 = t6547 * t23265;
    let t82147 = t23030 * t23208;
    let t82149 = 0.49348022005446793095e-1 * t82120 - t82123 - 0.24674011002723396548e-1 * t82126 + 0.49348022005446793095e-1 * t82129 - 0.57572692339687925277e-1 * t82131 + 0.24674011002723396547e-1 * t82135 - 0.49348022005446793095e-1 * t82138 + 0.14804406601634037928e0 * t82141 + 0.57572692339687925277e-1 * t82143 + 0.11514538467937585055e0 * t82145 - 0.78134368175290755733e-1 * t82147;
    (t82149,)
}
