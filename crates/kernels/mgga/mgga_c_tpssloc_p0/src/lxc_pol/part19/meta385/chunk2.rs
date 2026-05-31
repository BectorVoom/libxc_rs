//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1444/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1444<F: Float>(t43776: F, t43837: F, t43839: F, t43842: F, t43845: F, t43848: F, t43851: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F) -> F {
    let t44466 = F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t43776;
    let t44470 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43837 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43839 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43842 + F::cast_from(2.0_f64) * t43845 - F::cast_from(4.0_f64) * t43848 - t43851 / F::cast_from(6.0_f64) + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t43855 + F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t43857 - t44466 + F::cast_from(160.0_f64) / F::cast_from(81.0_f64) * t43859 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t43861 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t43863;
    t44470
}
