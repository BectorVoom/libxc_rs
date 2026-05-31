//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1401/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1401<F: Float>(t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t11778: F, t154: F) -> (F, F) {
    let t43808 = -F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t43748 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t43750 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43780 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t43782 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t43784 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43786 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43788 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t43794 - F::cast_from(8.0_f64) * t43798 + F::cast_from(8.0_f64) * t43802 + t43806 / F::cast_from(3.0_f64);
    let t43809 = t154 * t11778;
    (t43808, t43809)
}
