//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2768/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2768<F: Float>(t13084: F, t16836: F, t16839: F, t16891: F, t16896: F, t16898: F, t16901: F, t2633: F, t2643: F, t2645: F, t2679: F, t2684: F, t40966: F, t40982: F, t40990: F, t4178: F, t4180: F, t4240: F, t47044: F, t58353: F, t58363: F, t58373: F, t58379: F, t58381: F, t9642: F, t9646: F, t9647: F) -> F {
    let t58392 = -t4178 * t2645 * t16901 * t2633 / F::new(384.0) + F::new(7.0) / F::new(384.0) * t58353 - F::new(5.0) / F::new(768.0) * t2643 * t9646 * t16891 * t9647 - t2643 * t4180 * t16839 * t2684 / F::new(3072.0) + F::new(7.0) / F::new(2304.0) * t58363 - t16836 * t13084 / F::new(192.0) + F::new(7.0) / F::new(1536.0) * t4178 * t4180 * t16839 * t2633 - t47044 * t4240 / F::new(768.0) - F::new(7.0) / F::new(288.0) * t58373 - F::new(5.0) / F::new(768.0) * t2643 * t9646 * t16839 * t9647 - F::new(7.0) / F::new(288.0) * t58379 + F::new(7.0) / F::new(1152.0) * t58381 - F::new(5.0) / F::new(384.0) * t9642 * t16898 - F::new(5.0) / F::new(768.0) * t2643 * t9646 * t16896 * t2679 + F::new(595.0) / F::new(1296.0) * t40966 - F::new(119.0) / F::new(3456.0) * t40982 + F::new(595.0) / F::new(3456.0) * t40990;
    t58392
}
