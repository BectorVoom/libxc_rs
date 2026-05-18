//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1220/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1220<F: Float>(t20034: F, t20062: F, t1390: F, t6463: F, t12044: F, t12048: F, t12057: F, t12059: F, t1297: F, t1307: F, t1388: F, t15898: F, t15911: F, t15916: F, t15917: F, t15923: F, t193: F, t19596: F, t19599: F, t19603: F, t19631: F, t19677: F, t3918: F, t5126: F, t5160: F, t5161: F, t533: F, t5356: F, t571: F, t6330: F, t9780: F, t9789: F) -> F {
    let t20063 = t20034 + t20062;
    let t20067 = t6463 * t1390;
    let t20075 = -t15898 + t9780 - t5160 * t19596 * t1388 + t19599 + t12044 + t15911 - t12048 - F::new(2.0) * t5160 * t5161 * t5356 + F::new(12.0) * t5126 * t19603 + F::new(3.0) * t193 * t1297 * t19631 + t193 * t533 * t20063 * t1390 + F::new(3.0) * t3918 * t20067 * t1307 + t19677 - t15916 - t15917 - t12057 + F::new(6.0) * t193 * t1307 * t571 * t6330 - t12059 + t15923 - t9789;
    t20075
}
