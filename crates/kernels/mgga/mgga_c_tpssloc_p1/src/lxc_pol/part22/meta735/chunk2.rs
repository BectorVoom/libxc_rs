//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2415/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2415<F: Float>(t1637: F, t17198: F, t4696: F, t4700: F, t60867: F, t68905: F, t68910: F, t68912: F, t68916: F, t68918: F, t68920: F, t68923: F, t68926: F, t68930: F) -> F {
    let t68931 = -F::new(3.0) * t1637 * t4700 * t60867 + F::new(6.0) * t17198 * t4696 * t4700 - t68905 + t68910 - t68912 + t68916 + t68918 - t68920 - t68923 + t68926 - t68930;
    t68931
}
