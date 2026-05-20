//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2282/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2282<F: Float>(t16596: F, t89992: F, t23788: F, t98007: F, t17109: F, t28: F, t25365: F, t98058: F, t25927: F, t98003: F, t1081: F, t1877: F, t22959: F, t23290: F, t25013: F, t2522: F, t25354: F, t25358: F, t25930: F, t25934: F, t28448: F, t28774: F, t28792: F, t28795: F, t6666: F, t6670: F, t7649: F, t7656: F, t86836: F, t99055: F) -> F {
    let t100766 = t89992 * t16596;
    let t100769 = t23788 * t98007;
    let t100772 = t28 * t17109;
    let t100780 = t89992 * t25365;
    let t100788 = t23788 * t98058;
    let t100791 = t25927 * t98003;
    let t100803 = -t1877 * t86836 * t7656 - F::new(3.0) * t22959 * t100766 - F::new(3.0) * t22959 * t100769 - t1877 * t6670 * t100772 / F::new(2.0) - t1877 * t25358 * t25934 - t1877 * t25358 * t25930 - F::new(3.0) * t22959 * t100780 + F::new(3.0) * t2522 * t25354 * t7649 - t1877 * t23290 * t28792 - t99055 - F::new(6.0) * t25013 * t100788 + F::new(3.0) * t22959 * t100791 + t1877 * t28448 * t1081 / F::new(2.0) + F::new(3.0) * t2522 * t6666 * t28774 - t1877 * t23290 * t28795 / F::new(2.0);
    t100803
}
