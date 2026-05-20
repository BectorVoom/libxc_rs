//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1434/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1434<F: Float>(t43819: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43811: F, t43816: F, t43823: F, t43828: F) -> F {
    let t44275 = F::cast_from(0.5356037037037037037e1_f64) * t43819;
    let t44289 = t44275 + F::cast_from(0.13772666666666666666e1_f64) * t43780 + F::cast_from(0.27545333333333333333e1_f64) * t43782 + F::cast_from(0.27545333333333333332e1_f64) * t43784 - F::new(0.41318e1) * t43786 - F::cast_from(0.68863333333333333332e0_f64) * t43788 + F::cast_from(0.68863333333333333334e1_f64) * t43794 - F::new(0.123954e2) * t43798 + F::new(0.123954e2) * t43802 + F::new(0.516475e0) * t43806 - F::cast_from(0.15302962962962962963e1_f64) * t43811 - F::cast_from(0.21424148148148148148e1_f64) * t43816 - F::new(0.103295e1) * t43823 + F::new(0.309885e1) * t43828;
    t44289
}
