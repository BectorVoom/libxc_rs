//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2111/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2111<F: Float>(t13336: F, t1898: F, t249: F, t23047: F, t4166: F, t2635: F, t81736: F, t81743: F, t81750: F, t87183: F, t87185: F, t87187: F, t87189: F, t87191: F, t87193: F, t87195: F, t87198: F, t87200: F, t87206: F, t87212: F, t87213: F) -> F {
    let t87216 = t13336 * t1898 * t249;
    let t87218 = t4166 * t23047;
    let t87219 = t87218 * t2635;
    let t87221 = -t87183 / F::new(768.0) + t87185 / F::new(192.0) + t87187 / F::new(192.0) + t87189 / F::new(192.0) + t87191 / F::new(192.0) - t87193 / F::new(1536.0) - t87195 / F::new(768.0) - t87198 + t87200 / F::new(192.0) - t87206 - t81736 + t81743 - F::new(7.0) / F::new(288.0) * t81750 + t87212 + F::cast_from(0.16821981705891829522e-4_f64) * t87213 + t87216 / F::new(1536.0) + t87219 / F::new(768.0);
    t87221
}
