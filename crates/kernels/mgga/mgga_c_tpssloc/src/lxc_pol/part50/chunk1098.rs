//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1098/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1098<F: Float>(t1983: F, t33136: F, t6996: F, t652: F, t6862: F, t7467: F, t2314: F, t32670: F, t32782: F, t6999: F, t33133: F, t6997: F, t24987: F, t8490: F, t1437: F, t31: F) -> (F, F, F, F, F, F, F) {
    let t119867 = 2.0 * t1983 * t6996 * t33136;
    let t119869 = t652 * t6862 * t7467;
    let t119871 = t2314 * t32670;
    let t119874 = t1983 * t32782 * t6999;
    let t119875 = t33133 * t6997;
    let t119877 = t24987 * t8490;
    let t119878 = t1437 * t31;
    (t119867, t119869, t119871, t119874, t119875, t119877, t119878)
}
