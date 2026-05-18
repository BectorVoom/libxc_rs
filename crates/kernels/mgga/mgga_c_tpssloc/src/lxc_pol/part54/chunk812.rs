//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 812/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk812<F: Float>(t1375: F, t1843: F, t2016: F, t5215: F, t5321: F, t568: F, t6885: F, t6900: F, t6958: F, t7693: F, t7698: F, t7702: F, t7704: F, t7723: F, t7729: F, t7750: F) -> F {
    let t7752 = -t6885 - F::new(0.16449340668482264365e-1) * t7693 - t6900 + F::new(0.82246703342411321825e-2) * t7698 - F::new(0.82246703342411321825e-2) * t7702 + t7704 * t568 + t7723 * t568 - t6958 * t1843 - t5215 * t2016 - t5321 * t2016 + F::new(2.0) * t1375 * t7729 - t1375 * t7750;
    t7752
}
