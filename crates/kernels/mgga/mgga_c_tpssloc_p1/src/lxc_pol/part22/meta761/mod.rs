//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta761 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2562;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta761<F: Float>(t3264: F, t4782: F, t6020: F, t1671: F, t18834: F, t11185: F, t21899: F, t1670: F, t3313: F, t63588: F, t18258: F, t4781: F, t14850: F, t18677: F, t14838: F, t18680: F, t15207: F, t18640: F, t4802: F, t4824: F, t64103: F, t64292: F, t71793: F, t71795: F, t71797: F, t71800: F, t71803: F) -> (F, F, F, F, F, F, F, F) {
        let (t71806, t71809, t71811, t71814, t71817) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2562::<F>(t3264, t4782, t6020, t1671, t18834, t11185, t21899, t1670, t3313, t63588, t18258, t4781);
        let (t71819, t71821, t71828) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2563::<F>(t14850, t18677, t14838, t18680, t15207, t18640, t4802, t4824, t64103, t64292, t71793, t71795, t71797, t71800, t71803, t71806, t71809, t71811, t71814, t71817);
    (t71806, t71809, t71811, t71814, t71817, t71819, t71821, t71828)
}
