//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1151/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1151<F: Float>(t1873: F, t3652: F, t652: F, t6876: F, t7000: F, t6880: F, t9348: F, t12734: F, t2314: F, t6534: F, t12739: F, t5113: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23831 = t3652 * t1873;
    let t23833 = F::cast_from(2.0_f64) * t652 * t23831;
    let t23835 = F::cast_from(2.0_f64) * t6876 * t7000;
    let t23837 = F::cast_from(6.0_f64) * t6876 * t6880;
    let t23844 = F::cast_from(2.0_f64) * t9348 * t1873;
    let t23846 = F::cast_from(4.0_f64) * t12734 * t1873;
    let t23848 = F::cast_from(4.0_f64) * t2314 * t6534;
    let t23850 = F::cast_from(2.0_f64) * t12739 * t1873;
    let t23852 = F::cast_from(4.0_f64) * t5113 * t6534;
    (t23831, t23833, t23835, t23837, t23844, t23846, t23848, t23850, t23852)
}
