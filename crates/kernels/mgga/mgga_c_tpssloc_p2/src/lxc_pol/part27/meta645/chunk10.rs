//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2217/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2217<F: Float>(t25655: F, t82895: F, t25661: F, t1036: F, t25664: F, t1611: F, t23528: F, t23436: F, t4640: F, t14507: F, t23536: F, t1025: F, t1046: F, t1622: F, t23504: F, t25580: F, t25683: F, t3057: F, t3134: F, t378: F, t4616: F, t6758: F, t82868: F, t83080: F, t83082: F, t83098: F) -> F {
    let t88575 = F::cast_from(0.40372756094140390856e-3_f64) * t82895 * t25655;
    let t88577 = F::cast_from(0.20186378047070195428e-3_f64) * t82895 * t25661;
    let t88582 = t25664 * t1036 / F::new(1152.0);
    let t88584 = t1611 * t23528;
    let t88591 = t4640 * t23436;
    let t88594 = t14507 * t23536;
    let t88597 = t83080 - t83082 / F::new(216.0) + F::cast_from(0.10093189023535097714e-3_f64) * t25683 * t23504 + t88575 - t88577 - t4616 * t6758 * t378 / F::new(144.0) + t88582 + F::new(11.0) / F::new(324.0) * t83098 - t88584 * t1046 / F::new(216.0) + F::new(19.0) / F::new(1296.0) * t82868 * t1622 + t25580 * t3057 / F::new(2304.0) - t88591 * t1025 / F::new(144.0) + t88594 * t3134 / F::new(768.0);
    t88597
}
