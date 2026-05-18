//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1251/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1251<F: Float>(t794: F, t852: F, t213: F, t225: F, t1914: F, t40772: F, t1081: F, t2752: F, t1862: F, t607: F, t22573: F, t6875: F) -> (F, F, F, F, F, F) {
    let t82133 = t794 * t852;
    let t82159 = t213 * t852 * t225;
    let t82312 = t1914 * t40772;
    let t83555 = t2752 * t1081;
    let t83817 = t607 * t1862;
    let t83886 = t6875 * t22573;
    (t82133, t82159, t82312, t83555, t83817, t83886)
}
