//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2001/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2001<F: Float>(t26016: F, t92047: F, t2031: F, t96425: F, t23967: F, t27972: F, t27976: F, t2032: F, t23963: F, t23970: F, t26009: F, t26954: F, t83717: F, t90098: F, t90114: F, t91954: F, t92057: F, t96422: F, t96443: F, t96473: F, t96535: F) -> F {
    let t102173 = t26016 * t92047;
    let t102187 = t2031 * t96425;
    let t102192 = t23967 * t27972;
    let t102194 = t23967 * t27976;
    let t102198 = -F::new(160.0) / F::new(9.0) * t102173 + F::new(10.0) / F::new(3.0) * t96473 * t23970 + F::new(20.0) / F::new(3.0) * t26016 * t92057 + F::new(20.0) * t91954 * t26009 + F::new(20.0) / F::new(3.0) * t90114 * t26954 + F::new(20.0) / F::new(3.0) * t96443 * t23970 + F::new(20.0) * t23963 * t96422 - F::new(20.0) * t83717 * t102187 + F::new(20.0) / F::new(3.0) * t90098 * t26954 + F::new(80.0) / F::new(9.0) * t102192 + F::new(40.0) / F::new(9.0) * t102194 - F::new(2.0) / F::new(3.0) * t96535 * t2032;
    t102198
}
