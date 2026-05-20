//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2461/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2461<F: Float>(t1606: F, t2402: F, t973: F, t10454: F, t4644: F, t13950: F, t3117: F, t14202: F, t3048: F, t14206: F, t3108: F, t1025: F, t1041: F, t10501: F, t14085: F, t1622: F, t3064: F, t3098: F, t43374: F, t43377: F, t43382: F, t43406: F, t43410: F, t4582: F, t47775: F, t48497: F) -> F {
    let t50425 = t973 * t2402 * t1606;
    let t50429 = t4644 * t10454;
    let t50438 = t3117 * t13950;
    let t50442 = t3048 * t14202;
    let t50443 = t50442 / F::new(1296.0);
    let t50445 = t14206 * t3108;
    let t50452 = F::new(5.0) / F::new(3888.0) * t50425 - F::new(209.0) / F::new(3888.0) * t43410 * t1622 + t50429 / F::new(2304.0) - t43374 / F::new(144.0) + t43377 / F::new(216.0) + t43382 / F::new(3456.0) - t1041 * t4582 * t47775 * t48497 / F::new(192.0) + t50438 / F::new(1152.0) + F::new(5.0) / F::new(4608.0) * t14085 * t3064 + t50443 - F::new(5.0) / F::new(3456.0) * t43406 - t50445 * t1025 / F::new(96.0) - t14085 * t3098 / F::new(768.0) - F::new(5.0) / F::new(2304.0) * t4644 * t10501;
    t50452
}
