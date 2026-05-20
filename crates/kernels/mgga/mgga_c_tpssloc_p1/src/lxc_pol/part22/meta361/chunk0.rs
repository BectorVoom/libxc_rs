//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1598/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1598<F: Float>(t10556: F, t10577: F, t13598: F, t13600: F, t13601: F, t13603: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F, t17180: F, t17185: F, t17189: F) -> F {
    let t17271 = -t10577 - F::new(4.0) / F::new(27.0) * t10556 - F::new(8.0) / F::new(27.0) * t13598 + t13600 - t13601 + t13603 + F::new(2.0) / F::new(27.0) * t17149 - F::new(10.0) / F::new(27.0) * t17154 + F::new(4.0) / F::new(3.0) * t17159 - F::new(4.0) / F::new(9.0) * t17163 - F::new(2.0) / F::new(9.0) * t17165 - F::new(2.0) * t17169 + F::new(4.0) / F::new(3.0) * t17173 + t17175 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t17180 + F::new(2.0) / F::new(3.0) * t17185 - t17189 / F::new(3.0);
    t17271
}
