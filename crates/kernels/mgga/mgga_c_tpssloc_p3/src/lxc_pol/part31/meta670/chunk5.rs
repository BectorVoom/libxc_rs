//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1994/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1994<F: Float>(t100651: F, t100682: F, t100692: F, t100713: F, t100743: F, t101196: F, t101211: F, t101220: F, t101241: F, t1649: F, t1877: F, t2057: F, t24191: F, t2522: F, t25892: F, t25905: F, t25921: F, t26563: F, t26740: F, t26756: F, t28774: F, t28778: F, t7110: F, t7649: F, t7845: F, t92319: F) -> F {
    let t102012 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t100743 - F::cast_from(3.0_f64) * t26756 * t100682 + F::cast_from(6.0_f64) * t101196 * t25892 + t1877 * t26740 * t1649 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t100692 + F::cast_from(6.0_f64) * t24191 * t100713 + t101211 - t101220 - F::cast_from(3.0_f64) * t92319 * t25921 + F::cast_from(3.0_f64) * t2522 * t7110 * t28774 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7110 * t28778 + F::cast_from(3.0_f64) * t2522 * t7845 * t25905 + F::cast_from(3.0_f64) * t2522 * t26740 * t7649 - t101241 + F::cast_from(6.0_f64) * t26563 * t100651;
    t102012
}
