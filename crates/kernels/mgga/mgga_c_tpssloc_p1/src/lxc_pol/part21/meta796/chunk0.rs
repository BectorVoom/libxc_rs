//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2758/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2758<F: Float>(t40817: F, t13191: F, t13487: F, t16592: F, t16606: F, t17120: F, t1877: F, t193: F, t2378: F, t2522: F, t2553: F, t2749: F, t39549: F, t39563: F, t40772: F, t4307: F, t4310: F, t4314: F, t5664: F, t58071: F, t58080: F, t58085: F, t58090: F) -> (F, F) {
    let t58094 = F::cast_from(0.17315859105681463759e2_f64) * t40817;
    let t58095 = -F::new(6.0) * t1877 * t2749 * t40772 * t5664 + F::new(24.0) * t13191 * t4310 * t4314 + F::new(12.0) * t13487 * t17120 * t2522 + F::new(6.0) * t16592 * t2553 * t4314 + F::new(3.0) * t16606 * t2522 * t2553 + F::new(12.0) * t193 * t2378 * t58090 - F::new(12.0) * t2522 * t4307 * t58071 + t39549 + t39563 + t58080 + t58085 - t58094;
    (t58094, t58095)
}
