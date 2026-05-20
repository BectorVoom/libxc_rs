//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta760 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2560;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta760<F: Float>(t4745: F, t64257: F, t4786: F, t63755: F, t14838: F, t18255: F, t14850: F, t18259: F, t11303: F, t1136: F, t11361: F, t11420: F, t15146: F, t15207: F, t1683: F, t1694: F, t18615: F, t18623: F, t18631: F, t18634: F, t18893: F, t21839: F, t21842: F, t21952: F, t3332: F, t3357: F, t3401: F, t4819: F, t4820: F, t4857: F, t51376: F, t6037: F, t6052: F, t63533: F, t18683: F, t51249: F, t18262: F, t18266: F, t51120: F, t1117: F, t11275: F, t21961: F, t11190: F, t4781: F, t6024: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t71784, t71786, t71788, t71790, t71791) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2560::<F>(t4745, t64257, t4786, t63755, t14838, t18255, t14850, t18259, t11303, t1136, t11361, t11420, t15146, t15207, t1683, t1694, t18615, t18623, t18631, t18634, t18893, t21839, t21842, t21952, t3332, t3357, t3401, t4819, t4820, t4857, t51376, t6037, t6052, t63533);
        let (t71793, t71795, t71797, t71800, t71803) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2561::<F>(t18683, t51249, t14850, t18262, t18266, t51120, t1117, t11275, t21961, t11190, t4781, t6024);
    (t71784, t71786, t71788, t71790, t71791, t71793, t71795, t71797, t71800, t71803)
}
