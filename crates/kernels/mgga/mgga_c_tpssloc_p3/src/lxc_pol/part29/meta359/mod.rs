//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1453;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1454;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1455;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta359<F: Float>(t4300: F, t865: F, t2718: F, t2684: F, t4180: F, t4181: F, t9646: F, t9647: F, t2633: F, t2645: F, t4248: F, t1496: F, t9541: F, t12850: F, t12860: F, t12861: F, t12889: F, t12891: F, t12894: F, t12906: F, t12910: F, t9457: F, t9462: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t2427: F, t4202: F, t9869: F, t2655: F, t4205: F, t12914: F, t12922: F, t12926: F, t12927: F, t12928: F, t12934: F, t12942: F, t12944: F, t12947: F, t9724: F, t9780: F, t9789: F, t9863: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13071, t13072, t13076, t13080, t13084, t13087) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1453::<F>(t4300, t865, t2718, t2684, t4180, t4181, t9646, t9647, t2633, t2645, t4248, t1496, t9541);
        let t13093 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1454::<F>(t12850, t12860, t12861, t12889, t12891, t12894, t12906, t12910, t9457, t9462, t9469, t9476, t9484, t9496, t9715);
        let (t13095, t13096, t13098, t13099) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1455::<F>(t2427, t4202, t9869, t2655, t4205, t12914, t12922, t12926, t12927, t12928, t12934, t12942, t12944, t12947, t9724, t9780, t9789, t9863);
    (t13071, t13072, t13076, t13080, t13084, t13087, t13093, t13095, t13096, t13098, t13099)
}
