//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3202/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3202<F: Float>(t1244: F, t3068: F, t478: F, t6163: F, t11734: F, t1227: F, t15498: F, t15525: F, t15541: F, t19072: F, t3515: F, t3580: F, t4582: F, t4977: F, t4989: F, t5024: F, t52919: F, t53456: F, t53468: F, t53470: F, t53476: F, t53481: F, t53490: F, t53494: F, t53496: F, t53498: F, t61855: F) -> F {
    let t66622 = t1244 * t478 * t6163 * t3068;
    let t66631 = -F::new(2.0) / F::new(243.0) * t53456 - t53468 / F::new(3456.0) - t53470 / F::new(1728.0) - t53476 / F::new(864.0) - t53481 / F::new(864.0) - t11734 * t19072 / F::new(768.0) - t3515 * t4582 * t4977 * t15525 / F::new(1536.0) - F::new(5.0) / F::new(648.0) * t15498 * t4989 - F::new(5.0) / F::new(648.0) * t5024 * t15541 - F::new(5.0) / F::new(243.0) * t53490 - t53494 / F::new(1728.0) - F::new(19.0) / F::new(1296.0) * t66622 * t3580 + t53496 / F::new(162.0) + t53498 / F::new(81.0) + F::new(55.0) / F::new(15552.0) * t1227 * t4582 * t52919 * t61855;
    t66631
}
