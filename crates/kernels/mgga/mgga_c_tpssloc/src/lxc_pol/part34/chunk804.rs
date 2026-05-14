//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 804/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk804<F: Float>(t21089: F, t2929: F, t951: F, t959: F, t10523: F, t2932: F, t1589: F, t17934: F, t10629: F, t10632: F, t4483: F, t5808: F, t5812: F, t1568: F, t5742: F, t2888: F) -> (F, F, F, F, F, F, F, F) {
    let t21091 = t2929 * t21089 * t951;
    let t21093 = 0.35089341735807877242e1 * t959 * t21091;
    let t21094 = t10523 * t21089;
    let t21095 = t21094 * t2932;
    let t21097 = 0.10389515463408878255e3 * t959 * t21095;
    let t21099 = 0.17544670867903938621e1 * t17934 * t1589;
    let t21100 = t10629 * t21089;
    let t21101 = t21100 * t10632;
    let t21103 = 0.10254018858216406658e4 * t959 * t21101;
    let t21105 = 0.17544670867903938621e1 * t4483 * t5808;
    let t21107 = 0.51947577317044391276e2 * t4483 * t5812;
    let t21114 = t5742 * t1568;
    let t21115 = t21114 * t2888;
    (t21093, t21097, t21099, t21103, t21105, t21107, t21114, t21115)
}
