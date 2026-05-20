//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3008/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3008<F: Float>(t10403: F, t10422: F, t18015: F, t1036: F, t18010: F, t14025: F, t14508: F, t13970: F, t14511: F, t10263: F, t10408: F, t13546: F, t14222: F, t14228: F, t1616: F, t17156: F, t17637: F, t17643: F, t3048: F, t3070: F, t3071: F, t3088: F, t3151: F, t378: F, t43382: F, t49934: F, t50438: F, t50442: F, t55723: F, t5885: F, t5890: F, t5904: F, t973: F, t974: F) -> F {
    let t62891 = t10403 * t10422 * t18015;
    let t62893 = t18010 * t1036;
    let t62901 = t14508 * t14025;
    let t62903 = t14511 * t13970;
    let t62909 = -t49934 * t14222 / F::new(1152.0) - t3070 * t3071 * t1616 * t13546 / F::new(1152.0) - t973 * t974 * t3151 * t55723 / F::new(72.0) + F::new(11.0) / F::new(324.0) * t10263 * t5890 - F::new(11.0) / F::new(162.0) * t10263 * t5885 - F::new(5.0) / F::new(1152.0) * t3070 * t10408 * t17156 * t14228 + t62891 / F::new(864.0) - t62893 / F::new(432.0) + F::new(19.0) / F::new(1728.0) * t5904 * t3088 * t378 + t43382 / F::new(5184.0) + t50438 / F::new(1728.0) + t50442 / F::new(972.0) + t62901 / F::new(576.0) - t62903 / F::new(1152.0) + t3048 * t17637 / F::new(216.0) - F::new(5.0) / F::new(1296.0) * t3048 * t17643;
    t62909
}
