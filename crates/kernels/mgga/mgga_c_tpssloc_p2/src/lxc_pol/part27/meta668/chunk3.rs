//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2358/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2358<F: Float>(t1983: F, t22591: F, t24990: F, t24987: F, t6880: F, t22573: F, t7684: F, t22575: F, t22585: F, t7685: F, t12725: F, t12813: F, t1976: F, t22483: F, t2312: F, t2314: F, t2323: F, t24983: F, t24999: F, t25958: F, t3652: F, t4026: F, t4028: F, t650: F, t652: F, t6539: F, t671: F, t6862: F, t7451: F, t7670: F, t91623: F, t91625: F, t91627: F, t91630: F, t91637: F) -> F {
    let t91640 = F::new(6.0) * t1983 * t22591 * t24990;
    let t91642 = F::new(6.0) * t24987 * t6880;
    let t91655 = t7684 * t22573;
    let t91657 = F::new(6.0) * t91655 * t22575;
    let t91662 = F::new(3.0) * t7685 * t22585;
    let t91663 = -F::new(2.0) * t12813 * t1976 * t652 - F::new(4.0) * t25958 * t652 * t671 - F::new(4.0) * t12725 * t6539 - F::new(2.0) * t22483 * t4028 - t2312 * t7670 - F::new(4.0) * t2314 * t24983 - F::new(4.0) * t2323 * t24999 - F::new(2.0) * t25958 * t650 - t3652 * t7451 - F::new(2.0) * t4026 * t6862 + t91623 - t91625 - t91627 - t91630 + t91637 + t91640 + t91642 - t91657 + t91662;
    t91663
}
