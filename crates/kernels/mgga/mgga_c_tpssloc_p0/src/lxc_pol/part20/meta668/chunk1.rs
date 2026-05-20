//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2514/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2514<F: Float>(t11244: F, t1661: F, t43880: F, t43889: F, t14808: F, t3279: F, t11258: F, t4748: F, t14813: F, t4764: F, t11265: F, t3271: F, t4756: F) -> (F, F, F, F, F, F, F) {
    let t51007 = t43880 * t1661 * t11244;
    let t51010 = t43889 * t1661 * t11244;
    let t51012 = t14808 * t3279;
    let t51014 = t4748 * t11258;
    let t51016 = t14813 * t3279;
    let t51018 = t4764 * t11258;
    let t51021 = t11265 * t4756 * t3271;
    (t51007, t51010, t51012, t51014, t51016, t51018, t51021)
}
