//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 741/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk741<F: Float>(t22779: F, t7712: F, t1887: F, t22839: F, t1377: F, t1799: F, t22674: F, t7700: F, t6897: F, t6883: F, t7697: F, t225: F, t7723: F) -> (F, F, F, F, F, F) {
    let t26295 = t22779 * t7712;
    let t26331 = t22839 * t1887;
    let t26337 = t1377 * t1799;
    let t26344 = t22674 * t7700;
    let t26345 = t6897 * t26344;
    let t26361 = t6883 * t7697;
    let t26366 = t7723 * t225;
    (t26295, t26331, t26337, t26345, t26361, t26366)
}
