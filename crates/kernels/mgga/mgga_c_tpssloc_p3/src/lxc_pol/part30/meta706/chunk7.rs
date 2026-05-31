//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2327/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2327<F: Float>(t16596: F, t89992: F, t23788: F, t98007: F, t17109: F, t28: F, t25365: F, t98058: F, t25927: F, t98003: F, t1081: F, t1877: F, t22959: F, t23290: F, t25013: F, t2522: F, t25354: F, t25358: F, t25930: F, t25934: F, t28448: F, t28774: F, t28792: F, t28795: F, t6666: F, t6670: F, t7649: F, t7656: F, t86836: F, t99055: F) -> F {
    let t100766 = t89992 * t16596;
    let t100769 = t23788 * t98007;
    let t100772 = t28 * t17109;
    let t100780 = t89992 * t25365;
    let t100788 = t23788 * t98058;
    let t100791 = t25927 * t98003;
    let t100803 = -t1877 * t86836 * t7656 - F::cast_from(3.0_f64) * t22959 * t100766 - F::cast_from(3.0_f64) * t22959 * t100769 - t1877 * t6670 * t100772 / F::cast_from(2.0_f64) - t1877 * t25358 * t25934 - t1877 * t25358 * t25930 - F::cast_from(3.0_f64) * t22959 * t100780 + F::cast_from(3.0_f64) * t2522 * t25354 * t7649 - t1877 * t23290 * t28792 - t99055 - F::cast_from(6.0_f64) * t25013 * t100788 + F::cast_from(3.0_f64) * t22959 * t100791 + t1877 * t28448 * t1081 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2522 * t6666 * t28774 - t1877 * t23290 * t28795 / F::cast_from(2.0_f64);
    t100803
}
