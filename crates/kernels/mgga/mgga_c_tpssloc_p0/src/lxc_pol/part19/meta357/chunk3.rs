//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1297/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1297<F: Float>(t41646: F, t41651: F, t41680: F, t41695: F, t41707: F, t41713: F, t41717: F, t41882: F, t41885: F, t41887: F, t41889: F, t41892: F, t41927: F, t41929: F) -> F {
    let t42203 = -F::cast_from(0.10805407407407407407e0_f64) * t41882 - F::new(0.104195e0) * t41885 - F::new(0.166712e1) * t41887 + F::cast_from(0.27785333333333333334e0_f64) * t41889 + F::new(0.125034e1) * t41892 + F::new(0.41318e1) * t41646 + F::new(0.123954e2) * t41651 + F::cast_from(0.13772666666666666666e1_f64) * t41680 - F::cast_from(0.34431666666666666667e1_f64) * t41695 - F::cast_from(0.13772666666666666667e1_f64) * t41707 - F::new(0.41318e1) * t41713 - F::new(0.185931e2) * t41717 + F::new(0.3529725e1) * t41927 + F::new(0.6311625e0) * t41929;
    t42203
}
