//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2491/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2491<F: Float>(t21126: F, t2970: F, t973: F, t1023: F, t1031: F, t13995: F, t17677: F, t21130: F, t21482: F, t21490: F, t21493: F, t2960: F, t3070: F, t378: F, t42397: F, t43307: F, t4579: F, t50362: F, t61950: F, t62811: F, t62816: F) -> F {
    let t70867 = t973 * t2970 * t21126;
    let t70884 = -t50362 + t2960 * t21490 / F::new(18.0) - t70867 / F::new(144.0) - t2960 * t21493 / F::new(27.0) - t43307 - t21482 * t1031 * t378 / F::new(576.0) + t13995 * t17677 / F::new(768.0) + t62811 / F::new(2304.0) + F::new(5.0) / F::new(5184.0) * t3070 * t42397 * t21130 * t1023 + t61950 * t4579 / F::new(1536.0) + t62816 / F::new(1536.0);
    t70884
}
