//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 779/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk779<F: Float>(t1528: F, t259: F, t2597: F, t2713: F, t4143: F, t4145: F, t4147: F, t4149: F, t4266: F, t4268: F, t4273: F, t4301: F, t855: F, t866: F) -> F {
    let t4303 = -t1528 * t2597 - t1528 * t2713 + t259 * t4143 + t259 * t4145 + t259 * t4149 + t259 * t4266 - t4147 * t866 - t4268 * t866 + F::new(2.0) * t4273 * t855 - t4301 * t855;
    t4303
}
