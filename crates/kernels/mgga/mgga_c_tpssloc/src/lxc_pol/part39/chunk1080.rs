//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1080/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1080<F: Float>(t15026: F, t3609: F, t11877: F, t11881: F, t1244: F, t1249: F, t14986: F, t14989: F, t14992: F, t14997: F, t15001: F, t15004: F, t15009: F, t15016: F, t15019: F, t15023: F, t1729: F, t1756: F, t3604: F, t3610: F, t3613: F, t3617: F, t3624: F, t3628: F, t4964: F, t5064: F, t5073: F) -> (F,) {
    let t15027 = t15026 * t3609;
    let t15030 = t11877 * t1756 + 6.0 * t11881 * t15001 + t1244 * t14986 + 2.0 * t1244 * t14989 + t1244 * t15016 + 2.0 * t1249 * t4964 - 2.0 * t14992 * t3624 + 4.0 * t14997 * t3610 + 4.0 * t15004 * t3610 + 2.0 * t15009 * t3610 - t15019 * t3624 - t15023 * t3624 + 2.0 * t15027 * t3613 + t1729 * t3628 + 2.0 * t3604 * t5073 + 2.0 * t3617 * t5064;
    (t15030,)
}
