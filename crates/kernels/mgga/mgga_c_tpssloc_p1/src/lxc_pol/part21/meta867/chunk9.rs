//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3173/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3173<F: Float>(t15486: F, t5005: F, t1222: F, t18574: F, t1174: F, t15527: F, t1748: F, t19033: F, t3440: F, t3527: F, t3531: F, t3587: F, t5019: F, t53487: F, t63390: F, t65660: F, t65662: F, t65664: F, t65668: F, t65670: F, t65672: F, t65674: F) -> F {
    let t65676 = t5005 * t15486;
    let t65681 = t18574 * t1222;
    let t65685 = -F::new(19.0) / F::new(2592.0) * t19033 * t3527 - F::new(19.0) / F::new(1296.0) * t19033 * t3531 - t5019 * t15527 / F::new(288.0) + t65660 / F::new(1152.0) + F::new(5.0) / F::new(10368.0) * t65662 - F::new(19.0) / F::new(7776.0) * t65664 - t53487 * t1748 / F::new(2304.0) + t65668 / F::new(324.0) + F::new(19.0) / F::new(1296.0) * t65670 - F::new(19.0) / F::new(1944.0) * t65672 - t65674 / F::new(2304.0) - t65676 / F::new(1728.0) + t1174 * t3440 * t63390 / F::new(6.0) + t65681 / F::new(2304.0) + F::new(95.0) / F::new(7776.0) * t19033 * t3587;
    t65685
}
