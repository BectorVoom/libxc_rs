//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1321/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1321<F: Float>(t1888: F, t23270: F, t25044: F, t5657: F, t1880: F, t25224: F, t28263: F, t105419: F, t105423: F, t105428: F, t17092: F, t21034: F, t218: F, t259: F, t28432: F, t4268: F, t6627: F, t7538: F, t86955: F, t86991: F, t98237: F) -> F {
    let t105437 = t1888 * t23270 * t25044 * t5657;
    let t105441 = t1880 * t25224 * t28263;
    let t105443 = -F::cast_from(0.74022033008170189643e-1_f64) * t98237 + F::cast_from(0.19190897446562641759e0_f64) * t86955 + t218 * t105419 * t259 - F::cast_from(0.49348022005446793095e-1_f64) * t105423 + F::cast_from(0.82246703342411321825e-2_f64) * t105428 - F::cast_from(3.0_f64) * t4268 * t28432 - t6627 * t21034 - F::cast_from(6.0_f64) * t17092 * t7538 + F::cast_from(0.49348022005446793095e-1_f64) * t105437 - F::cast_from(0.19190897446562641759e0_f64) * t86991 - F::cast_from(0.24674011002723396548e-1_f64) * t105441;
    t105443
}
