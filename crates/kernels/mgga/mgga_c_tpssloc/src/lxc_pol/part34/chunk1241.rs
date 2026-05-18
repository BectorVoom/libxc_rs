//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1241/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1241<F: Float>(t102227: F, t102275: F, t106813: F, t106826: F, t106829: F, t106842: F, t106849: F, t106853: F, t106855: F, t2032: F, t23963: F, t26911: F, t27961: F, t27976: F, t27982: F, t7026: F, t7432: F, t7782: F, t91954: F) -> F {
    let t108708 = F::new(30.0) * t91954 * t27961 + F::new(30.0) * t23963 * t106826 - F::new(5.0) * t7026 * t106813 - F::new(5.0) * t7026 * t106842 - F::new(5.0) * t26911 * t27976 - F::new(5.0) / F::new(3.0) * t7026 * t106849 - F::new(2.0) * t102227 * t106853 + F::new(10.0) * t102275 * t7432 - F::new(2.0) * t106829 * t2032 - F::new(2.0) / F::new(3.0) * t106855 * t2032 - F::new(2.0) * t27982 * t7782;
    t108708
}
