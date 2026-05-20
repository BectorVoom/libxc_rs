//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3013/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3013<F: Float>(t1058: F, t1060: F, t11051: F, t14526: F, t14600: F, t14618: F, t1615: F, t18086: F, t18099: F, t18155: F, t23508: F, t3040: F, t3180: F, t3197: F, t3200: F, t3201: F, t360: F, t43503: F, t43515: F, t43516: F, t43576: F, t43577: F, t4594: F, t4649: F, t4674: F, t4684: F, t4685: F, t50465: F, t50509: F, t50516: F, t50592: F, t5928: F, t5937: F, t62925: F) -> F {
    let t63058 = F::new(24.0) * t43576 * t5928 * t43577 * t3040 + t18086 * t3197 + t11051 * t5937 - F::new(24.0) * t50516 * t50509 * t4594 * t4649 + F::new(14.0) * t43515 * t5928 * t43516 * t3040 + F::new(2.0) * t3180 * t18155 - F::new(2.0) * t3200 * t18099 * t4684 + F::new(8.0) * t14618 * t14600 + F::new(2.0) * t1058 * t14526 * t1615 * t1060 - t43503 * t5928 * t23508 * t3040 * t360 - F::new(4.0) * t50592 * t4685 - t3200 * t62925 * t3201 + F::new(8.0) * t50465 * t4674;
    t63058
}
