//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1207/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1207<F: Float>(t1036: F, t32954: F, t25628: F, t8384: F, t30828: F, t4640: F, t1611: F, t30839: F, t23472: F, t6753: F, t7582: F, t1025: F, t1046: F, t113416: F, t113418: F, t113429: F, t113432: F, t1610: F, t25683: F, t30821: F, t30832: F, t378: F, t4615: F, t8387: F) -> F {
    let t119331 = t32954 * t1036;
    let t119335 = t25628 * t8384;
    let t119337 = t4640 * t30828;
    let t119340 = t1611 * t30839;
    let t119346 = t23472 * t6753 * t7582;
    let t119349 = t4615 * t8387 * t378 / F::new(1536.0) - t1610 * t30832 * t378 / F::new(288.0) + t119331 / F::new(2304.0) + F::cast_from(0.40372756094140390856e-3_f64) * t25683 * t30821 + F::cast_from(0.40372756094140390856e-3_f64) * t119335 + t119337 * t1025 / F::new(1536.0) + t119340 * t1046 / F::new(2304.0) - F::cast_from(0.32298204875312312685e-2_f64) * t113416 + F::cast_from(0.40372756094140390856e-3_f64) * t113418 + F::cast_from(0.40372756094140390856e-3_f64) * t119346 - t113429 / F::new(432.0) - t113432;
    t119349
}
