//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 675/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk675<F: Float>(t1375: F, t1386: F, t3753: F, t3755: F, t3758: F, t3880: F, t3882: F, t3889: F, t3912: F, t568: F) -> F {
    let t3914 = F::cast_from(2.0_f64) * t1375 * t3889 - t1375 * t3912 - F::cast_from(2.0_f64) * t1386 * t3758 - F::cast_from(2.0_f64) * t1386 * t3882 + t3753 * t568 + F::cast_from(2.0_f64) * t3755 * t568 + t3880 * t568;
    t3914
}
