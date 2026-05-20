//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1465/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1465<F: Float>(t103103: F, t105131: F, t105144: F, t105146: F, t105147: F, t105150: F, t109982: F, t110002: F, t1398: F, t1852: F, t1858: F, t2170: F, t2174: F, t22431: F, t22453: F, t29866: F, t29884: F, t3: F, t580: F, t6471: F, t6483: F, t8111: F, t8119: F) -> F {
    let tv4rho3sigma11 = t109982 * t3 * t580 + t110002 * t1398 + F::new(3.0) * t1852 * t29884 + F::new(3.0) * t1858 * t29866 + t2170 * t22453 + t2174 * t22431 + F::new(3.0) * t6471 * t8119 + F::new(3.0) * t6483 * t8111 + F::new(3.0) * t103103 + F::new(6.0) * t105131 + F::new(6.0) * t105144 + F::new(3.0) * t105146 + F::new(3.0) * t105147 + F::new(3.0) * t105150;
    tv4rho3sigma11
}
