//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2465/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2465<F: Float>(t1022: F, t1058: F, t1060: F, t1061: F, t11037: F, t11046: F, t11051: F, t11078: F, t14526: F, t14595: F, t14627: F, t14630: F, t14645: F, t14651: F, t3180: F, t3186: F, t3188: F, t3197: F, t4669: F, t4673: F, t4677: F, t4678: F, t4680: F, t50535: F, t50540: F) -> F {
    let t50560 = F::new(3.0) * t1022 * t1058 * t1060 * t14526 + F::new(3.0) * t11046 * t14630 * t4677 + F::new(3.0) * t11046 * t14630 * t4680 + F::new(6.0) * t14595 * t3186 * t4673 + F::new(6.0) * t3186 * t3188 * t50540 + F::new(3.0) * t1061 * t50535 - F::new(3.0) * t11037 * t14627 + F::new(3.0) * t11051 * t4678 + F::new(3.0) * t11078 * t4669 + F::new(6.0) * t14645 * t3180 + F::new(3.0) * t14651 * t3197;
    t50560
}
