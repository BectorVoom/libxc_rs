//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 590/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk590<F: Float>(t2764: F, t273: F, t2799: F, t2807: F, t901: F, t241: F, t63: F, t281: F, t283: F, t699: F, t909: F) -> (F, F, F, F, F, F, F, F) {
    let t2810 = F::new(0.39862222222222222223e0) * t2764;
    let t2815 = F::new(1.0)/f64::sqrt(t273);
    let t2816 = t2815 * t2799;
    let t2818 = t901 * t2807;
    let t2820 = t63 * t241;
    let t2822 = t281 * t2820 * t283;
    let t2823 = F::new(0.13692777777777777778e0) * t2822;
    let t2824 = t699 * t909;
    (t2810, t2815, t2816, t2818, t2820, t2822, t2823, t2824)
}
