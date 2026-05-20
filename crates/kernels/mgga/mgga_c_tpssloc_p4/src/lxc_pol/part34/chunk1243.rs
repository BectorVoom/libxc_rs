//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1243/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1243<F: Float>(t102192: F, t102194: F, t102215: F, t102217: F, t102219: F, t102221: F, t102248: F, t106816: F, t2032: F, t26954: F, t27979: F, t7782: F, t91996: F, t96443: F) -> F {
    let t108743 = -F::new(2.0) * t106816 * t2032 - F::new(2.0) * t27979 * t7782 + F::new(80.0) / F::new(3.0) * t102192 + F::new(40.0) / F::new(3.0) * t102194 + F::new(16.0) / F::new(3.0) * t102215 + F::new(32.0) / F::new(3.0) * t102217 + F::new(80.0) / F::new(3.0) * t102219 + F::new(32.0) / F::new(3.0) * t102221 - F::new(80.0) * t102248 + F::new(88.0) / F::new(9.0) * t91996 + F::new(20.0) * t96443 * t26954;
    t108743
}
