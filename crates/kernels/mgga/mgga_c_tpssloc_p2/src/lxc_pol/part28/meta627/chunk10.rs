//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1965/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1965<F: Float>(t12835: F, t1459: F, t15857: F, t15868: F, t1774: F, t19456: F, t1983: F, t2039: F, t2040: F, t2095: F, t23909: F, t23917: F, t23918: F, t23938: F, t24432: F, t24987: F, t24995: F, t26179: F, t26872: F, t26977: F, t4028: F, t4037: F, t4077: F, t55169: F, t574: F, t652: F, t7042: F, t7057: F, t7217: F, t7220: F, t7458: F, t7802: F, t83886: F, t86685: F, t90381: F, t91854: F, t91857: F, t92099: F, t92139: F, t9348: F) -> F {
    let t92161 = -F::new(2.0) * t4028 * t23918 - F::new(2.0) * t90381 * t2040 - F::new(2.0) * t652 * t15857 * t2039 - F::new(4.0) * t91854 * t1459 - F::new(2.0) * t91857 * t1459 - F::new(4.0) * t26977 * t4037 - F::new(4.0) * t26179 * t7057 - F::new(2.0) * t7458 * t23909 - F::new(2.0) * t7042 * t12835 - F::new(4.0) * t23938 * t4077 + (t92099 + t92139) * t574 - t1983 * t2095 * t55169 - F::new(4.0) * t19456 * t7057 - F::new(2.0) * t652 * t1774 * t23917 - F::new(2.0) * t9348 * t7802 - F::new(2.0) * t1983 * t7217 * t15868 - F::new(6.0) * t83886 * t26872 - F::new(2.0) * t24987 * t7220 - F::new(12.0) * t24995 * t24432 * t86685;
    t92161
}
