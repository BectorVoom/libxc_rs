//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 914/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk914<F: Float>(t10311: F, t10318: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10589: F, t10591: F, t10597: F, t10600: F) -> F {
    let t10695 = F::cast_from(0.16431333333333333333e0_f64) * t10311 - F::cast_from(0.49293999999999999999e0_f64) * t10318 - F::cast_from(0.39862222222222222223e0_f64) * t10556 + F::cast_from(0.19931111111111111111e0_f64) * t10558 - F::cast_from(0.59793333333333333333e0_f64) * t10560 + F::cast_from(0.29896666666666666667e0_f64) * t10562 - F::cast_from(0.33218518518518518518e0_f64) * t10566 + F::cast_from(0.11958666666666666667e1_f64) * t10569 - F::new(0.17938e1) * t10572 - F::cast_from(0.29896666666666666667e0_f64) * t10575 + F::new(0.1898925e1) * t10589 + F::new(0.3071625e0) * t10591 + F::cast_from(0.142419375e1_f64) * t10597 - F::new(0.76790625e-1) * t10600;
    t10695
}
