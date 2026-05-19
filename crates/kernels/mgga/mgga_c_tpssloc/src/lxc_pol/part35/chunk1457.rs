//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1457/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1457<F: Float>(t103218: F, t103490: F, t104504: F, t104506: F, t104509: F, t104521: F, t104527: F, t1714: F, t1716: F, t21758: F, t21769: F, t22007: F, t27406: F, t27792: F, t27799: F, t29670: F, t29813: F, t6139: F, t6244: F, t7283: F, t7285: F, t7286: F, t7300: F, t8015: F, t85674: F, t85755: F, t86451: F, t95824: F) -> F {
    let t109844 = -F::cast_from(0.8529287754027840782e-2_f64) * t7283 * t85755 * t7286 * t21758 - F::cast_from(0.82246703342411321826e-2_f64) * t104504 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t7285 * t7286 * t21769 + F::cast_from(0.14621636149762012769e-1_f64) * t104506 - F::cast_from(0.54831135561607547883e-2_f64) * t104509 - F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t6139 * t1714 * t27799 + F::cast_from(0.82246703342411321826e-2_f64) * t104521 + F::cast_from(0.80418998823691070229e-1_f64) * t104527 + F::new(6.0) * t27792 * t6244 + F::cast_from(0.21932454224643019154e-1_f64) * t27406 * t29813 + t86451 - F::cast_from(0.49348022005446793095e-1_f64) * t7283 * t1716 * t103490 - F::cast_from(0.49348022005446793095e-1_f64) * t7283 * t7300 * t85674 * t22007 - F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t1716 * t29670 + F::cast_from(0.14621636149762012769e-1_f64) * t95824 - F::cast_from(0.24125699647107321069e0_f64) * t103218 * t8015;
    t109844
}
