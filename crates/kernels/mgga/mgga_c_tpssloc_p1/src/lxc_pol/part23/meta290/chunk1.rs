//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1005/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1005<F: Float>(t21126: F, t908: F, t136: F, t21122: F, t2826: F, t10577: F, t13598: F, t17149: F, t17165: F, t17175: F, t21124: F, t21128: F, t21147: F, t21150: F, t21153: F, t21156: F) -> (F, F, F, F, F) {
    let t21160 = t908 * t21126;
    let t21161 = t136 * t21160;
    let t21167 = t2826 * t21122;
    let t21168 = t136 * t21167;
    let t21180 = -t10577 - F::new(4.0) / F::new(9.0) * t13598 + F::new(2.0) / F::new(9.0) * t17149 - F::new(2.0) / F::new(3.0) * t17165 + t17175 / F::new(3.0) - F::new(10.0) / F::new(27.0) * t21147 + F::new(4.0) / F::new(3.0) * t21150 - F::new(2.0) / F::new(3.0) * t21124 - F::new(2.0) * t21153 + F::new(2.0) * t21128 - t21156 / F::new(3.0);
    (t21160, t21161, t21167, t21168, t21180)
}
