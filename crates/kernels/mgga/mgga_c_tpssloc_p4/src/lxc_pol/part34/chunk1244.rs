//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1244/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1244<F: Float>(t102163: F, t102168: F, t102187: F, t102303: F, t106758: F, t106800: F, t106804: F, t1860: F, t2031: F, t2032: F, t23963: F, t26016: F, t26954: F, t27937: F, t28935: F, t7428: F, t7782: F, t84280: F, t90137: F, t92003: F, t96473: F) -> F {
    let t108763 = F::new(10.0) * t96473 * t26954 + F::new(20.0) * t26016 * t102163 + F::new(10.0) * t26016 * t102168 + F::new(30.0) * t23963 * t106758 - F::new(60.0) * t90137 * t102187 + t106804 * t2032 / F::new(3.0) + t27937 * t7782 + t7428 * t28935 + t1860 * t2031 * t106800 / F::new(3.0) + F::new(88.0) / F::new(9.0) * t92003 - t84280 - F::new(8.0) / F::new(3.0) * t102303;
    t108763
}
