//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 748/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk748<F: Float>(t1375: F, t1386: F, t2016: F, t3758: F, t3882: F, t568: F, t6885: F, t6893: F, t6900: F, t6904: F, t6909: F, t6911: F, t6956: F, t6958: F, t6963: F, t6993: F) -> F {
    let t6995 = -t6885 - F::cast_from(0.16449340668482264365e-1_f64) * t6893 - t6900 + F::cast_from(0.82246703342411321825e-2_f64) * t6904 - F::cast_from(0.82246703342411321825e-2_f64) * t6909 + t6911 * t568 + t6956 * t568 - t6958 * t1386 - t3758 * t2016 - t3882 * t2016 + F::cast_from(2.0_f64) * t1375 * t6963 - t1375 * t6993;
    t6995
}
