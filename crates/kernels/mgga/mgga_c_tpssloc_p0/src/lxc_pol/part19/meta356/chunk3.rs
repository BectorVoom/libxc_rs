//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1290/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1290<F: Float>(t41646: F, t41651: F, t41680: F, t41695: F, t41707: F, t41713: F, t41717: F, t41882: F, t41885: F, t41887: F, t41889: F, t41892: F, t41927: F, t41929: F) -> F {
    let t42077 = -F::cast_from(0.85199506172839506175e-1_f64) * t41882 - F::cast_from(0.82156666666666666667e-1_f64) * t41885 - F::cast_from(0.13145066666666666666e1_f64) * t41887 + F::cast_from(0.21908444444444444444e0_f64) * t41889 + F::cast_from(0.98587999999999999999e0_f64) * t41892 + F::cast_from(0.23917333333333333333e1_f64) * t41646 + F::cast_from(0.71752000000000000001e1_f64) * t41651 + F::cast_from(0.79724444444444444444e0_f64) * t41680 - F::cast_from(0.19931111111111111111e1_f64) * t41695 - F::cast_from(0.79724444444444444444e0_f64) * t41707 - F::cast_from(0.23917333333333333333e1_f64) * t41713 - F::new(0.107628e2) * t41717 + F::new(0.1898925e1) * t41927 + F::new(0.3071625e0) * t41929;
    t42077
}
