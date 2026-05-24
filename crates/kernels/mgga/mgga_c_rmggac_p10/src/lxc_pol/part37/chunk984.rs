//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 984/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk984<F: Float>(t75729: F, t2211: F, t739: F, t8915: F, t699: F, t8712: F, t903: F, t15523: F, t2186: F, t71594: F, t14441: F, t5928: F) -> (F, F, F, F, F, F) {
    let t77804 = F::cast_from(0.16351352353374609375e-5_f64) * t75729;
    let t77806 = t739 * t2211 * t8915;
    let t77807 = F::cast_from(0.79828278012425390427e-1_f64) * t77806;
    let t77809 = t903 * t699 * t8712;
    let t77810 = F::cast_from(0.11974241701863808564e0_f64) * t77809;
    let t77811 = t2186 * t15523;
    let t77812 = F::cast_from(0.99317399751028291929e-5_f64) * t77811;
    let t77820 = F::cast_from(0.15243824895787514157e-3_f64) * t71594;
    let t77823 = F::cast_from(0.39914139006212695214e-1_f64) * t5928 * t14441;
    (t77804, t77807, t77810, t77812, t77820, t77823)
}
