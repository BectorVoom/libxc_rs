//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1444/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1444<F: Float>(t43776: F, t43837: F, t43839: F, t43842: F, t43845: F, t43848: F, t43851: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F) -> F {
    let t44466 = F::new(220.0) / F::new(81.0) * t43776;
    let t44470 = F::new(8.0) / F::new(3.0) * t43837 + F::new(4.0) / F::new(9.0) * t43839 - F::new(8.0) / F::new(9.0) * t43842 + F::new(2.0) * t43845 - F::new(4.0) * t43848 - t43851 / F::new(6.0) + F::new(10.0) / F::new(27.0) * t43855 + F::new(16.0) / F::new(81.0) * t43857 - t44466 + F::new(160.0) / F::new(81.0) * t43859 - F::new(10.0) / F::new(9.0) * t43861 - F::new(20.0) / F::new(9.0) * t43863;
    t44470
}
