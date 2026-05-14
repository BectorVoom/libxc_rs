//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1225/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1225<F: Float>(t1880: F, t25224: F, t28263: F, t105419: F, t105423: F, t105428: F, t105437: F, t17092: F, t21034: F, t218: F, t259: F, t28432: F, t4268: F, t6627: F, t7538: F, t86955: F, t86991: F, t98237: F) -> (F,) {
    let t105441 = t1880 * t25224 * t28263;
    let t105443 = -0.74022033008170189643e-1 * t98237 + 0.19190897446562641759e0 * t86955 + t218 * t105419 * t259 - 0.49348022005446793095e-1 * t105423 + 0.82246703342411321825e-2 * t105428 - 3.0 * t4268 * t28432 - t6627 * t21034 - 6.0 * t17092 * t7538 + 0.49348022005446793095e-1 * t105437 - 0.19190897446562641759e0 * t86991 - 0.24674011002723396548e-1 * t105441;
    (t105443,)
}
