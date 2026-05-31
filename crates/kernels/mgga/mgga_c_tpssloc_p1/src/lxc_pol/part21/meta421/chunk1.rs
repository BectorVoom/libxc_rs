//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1941/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1941<F: Float>(t11877: F, t11881: F, t1244: F, t1249: F, t14986: F, t14989: F, t14992: F, t14997: F, t15001: F, t15004: F, t15009: F, t15016: F, t15019: F, t15023: F, t15027: F, t1729: F, t1756: F, t3604: F, t3610: F, t3613: F, t3617: F, t3624: F, t3628: F, t4964: F, t5064: F, t5073: F) -> F {
    let t15030 = t11877 * t1756 + F::cast_from(6.0_f64) * t11881 * t15001 + t1244 * t14986 + F::cast_from(2.0_f64) * t1244 * t14989 + t1244 * t15016 + F::cast_from(2.0_f64) * t1249 * t4964 - F::cast_from(2.0_f64) * t14992 * t3624 + F::cast_from(4.0_f64) * t14997 * t3610 + F::cast_from(4.0_f64) * t15004 * t3610 + F::cast_from(2.0_f64) * t15009 * t3610 - t15019 * t3624 - t15023 * t3624 + F::cast_from(2.0_f64) * t15027 * t3613 + t1729 * t3628 + F::cast_from(2.0_f64) * t3604 * t5073 + F::cast_from(2.0_f64) * t3617 * t5064;
    t15030
}
