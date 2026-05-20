//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1205/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1205<F: Float>(t113372: F, t113392: F, t113438: F, t1615: F, t1618: F, t1622: F, t1935: F, t23483: F, t23535: F, t23564: F, t25608: F, t25645: F, t25652: F, t25654: F, t25660: F, t30816: F, t30817: F, t30821: F, t30829: F, t32948: F, t32951: F, t4652: F, t6730: F, t6753: F, t7574: F) -> F {
    let t119303 = F::cast_from(0.40372756094140390856e-3_f64) * t113372 - F::cast_from(0.40372756094140390856e-3_f64) * t1935 * t25608 * t30816 + t113392 * t1618 / F::new(1536.0) + t30829 * t4652 / F::new(1536.0) + t113438 * t1622 / F::new(2304.0) - F::cast_from(0.40372756094140390856e-3_f64) * t25645 * t30821 - F::cast_from(0.40372756094140390856e-3_f64) * t23564 * t32951 + F::cast_from(0.80745512188280781712e-3_f64) * t25652 * t23535 * t1615 * t25654 - F::cast_from(0.40372756094140390856e-3_f64) * t7574 * t30817 - F::cast_from(0.40372756094140390856e-3_f64) * t6730 * t32948 - F::cast_from(0.32298204875312312685e-2_f64) * t23483 * t32951 - F::cast_from(0.40372756094140390856e-3_f64) * t25652 * t6753 * t1615 * t25660;
    t119303
}
