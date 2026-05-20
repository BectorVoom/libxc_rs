//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2318/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2318<F: Float>(t18329: F, t7310: F, t1734: F, t18303: F, t18948: F, t18955: F, t19058: F, t19062: F, t19072: F, t19077: F, t24729: F, t24733: F, t27604: F, t27617: F, t478: F, t4974: F, t4980: F, t4984: F, t4989: F, t7345: F, t7376: F, t86146: F, t86171: F, t95270: F, t95273: F, t95303: F, t95304: F) -> F {
    let t104085 = t7310 * t18329;
    let t104087 = t24729 * t19058 / F::new(768.0) - t24733 * t19062 / F::new(1536.0) + t95270 * t4980 / F::new(384.0) - t95273 * t4984 / F::new(768.0) + F::new(5.0) / F::new(3456.0) * t27617 * t4989 + t24729 * t18948 / F::new(384.0) + t86146 * t18303 / F::new(256.0) + t27604 * t4974 / F::new(108.0) - F::new(5.0) / F::new(2592.0) * t7345 * t18955 - t24733 * t19072 / F::new(768.0) + t86171 * t19077 / F::new(1536.0) - F::cast_from(0.20186378047070195428e-3_f64) * t95303 * t95304 * t478 * t1734 * t7376 - t104085 / F::new(864.0);
    t104087
}
