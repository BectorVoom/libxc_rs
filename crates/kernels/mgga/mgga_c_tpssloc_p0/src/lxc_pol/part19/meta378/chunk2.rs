//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1413/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1413<F: Float>(t43819: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43811: F, t43816: F, t43823: F, t43828: F) -> F {
    let t43895 = F::cast_from(0.31310740740740740741e1_f64) * t43819;
    let t43909 = t43895 + F::cast_from(0.80513333333333333336e0_f64) * t43780 + F::cast_from(0.16102666666666666667e1_f64) * t43782 + F::cast_from(0.16102666666666666667e1_f64) * t43784 - F::cast_from(0.24154e1_f64) * t43786 - F::cast_from(0.40256666666666666668e0_f64) * t43788 + F::cast_from(0.40256666666666666666e1_f64) * t43794 - F::cast_from(0.72462e1_f64) * t43798 + F::cast_from(0.72462e1_f64) * t43802 + F::cast_from(0.301925e0_f64) * t43806 - F::cast_from(0.89459259259259259259e0_f64) * t43811 - F::cast_from(0.12524296296296296297e1_f64) * t43816 - F::cast_from(0.60384999999999999999e0_f64) * t43823 + F::cast_from(0.181155e1_f64) * t43828;
    t43909
}
