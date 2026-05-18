//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1295/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1295<F: Float>(t120092: F, t120095: F, t120097: F, t120103: F, t120107: F, t120111: F, t123178: F, t123180: F, t123182: F, t123184: F, t123187: F, t123189: F, t123193: F, t1442: F, t2165: F, t27293: F, t27371: F, t32572: F, t32605: F, t34372: F, t4028: F, t652: F, t671: F, t7264: F, t7266: F, t8103: F) -> F {
    let t125939 = -F::new(2.0) * t34372 * t652 * t671 - t1442 * t32572 - F::new(2.0) * t2165 * t27371 - F::new(4.0) * t27293 * t7266 - F::new(2.0) * t32605 * t4028 - F::new(2.0) * t7264 * t8103 - t120092 + t120095 - t120097 + t120103 + t120107 - t120111 - F::new(6.0) * t123178 - F::new(4.0) * t123180 - F::new(4.0) * t123182 - F::new(4.0) * t123184 - F::new(4.0) * t123187 + F::new(2.0) * t123189 + F::new(2.0) * t123193;
    t125939
}
