//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2980/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2980<F: Float>(t1041: F, t13969: F, t17696: F, t1021: F, t10390: F, t10413: F, t14211: F, t17681: F, t17688: F, t17925: F, t17976: F, t17991: F, t248: F, t2780: F, t2960: F, t2986: F, t3039: F, t3071: F, t3117: F, t360: F, t42546: F, t42610: F, t42613: F, t43361: F, t48477: F, t48611: F, t49757: F, t50366: F, t55677: F, t55716: F, t5878: F, t59659: F, t61719: F, t973: F, t974: F, t977: F, t998: F) -> F {
    let t62210 = t1041 * t13969 * t17696;
    let t62225 = -t3039 * t248 * t1021 * t61719 * t360 / F::new(1536.0) + t10390 * t17681 / F::new(2304.0) + t973 * t974 * t998 * t55677 / F::new(288.0) - t42610 / F::new(1296.0) - t42613 / F::new(972.0) + t49757 / F::new(2304.0) - t10413 * t3071 * t5878 * t2780 / F::new(4608.0) - t973 * t977 * t59659 / F::new(12.0) + F::new(2.0) / F::new(27.0) * t2960 * t17991 + F::new(5.0) / F::new(3888.0) * t62210 - t3117 * t17976 / F::new(576.0) - F::new(5.0) / F::new(1152.0) * t3117 * t17688 + t2986 * t50366 * t55716 / F::new(12.0) - t42546 * t17925 / F::new(1152.0) - t43361 * t48611 * t14211 * t48477 / F::new(128.0);
    t62225
}
