//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2357/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2357<F: Float>(t21064: F, t225: F, t13042: F, t13463: F, t1528: F, t17050: F, t17052: F, t17070: F, t21034: F, t252: F, t259: F, t2713: F, t4142: F, t4147: F, t4268: F, t4273: F, t4301: F, t5631: F, t5637: F, t5658: F, t59503: F, t68143: F, t866: F) -> F {
    let t68322 = t21064 * t225;
    let t68333 = t252 * t259 * t68143 + F::cast_from(3.0_f64) * t259 * t4142 * t5631 + F::cast_from(6.0_f64) * t13042 * t5637 + F::cast_from(6.0_f64) * t13463 * t5637 - F::cast_from(3.0_f64) * t13463 * t5658 - F::cast_from(3.0_f64) * t1528 * t59503 - F::cast_from(3.0_f64) * t17050 * t4147 + F::cast_from(6.0_f64) * t17052 * t4273 - F::cast_from(3.0_f64) * t17052 * t4301 + F::cast_from(12.0_f64) * t17070 * t4147 + F::cast_from(12.0_f64) * t17070 * t4268 - t21034 * t2713 - t68322 * t866;
    t68333
}
