//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1101/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1101<F: Float>(t2165: F, t4072: F, t671: F, t8103: F, t2109: F, t26012: F, t6509: F, t7974: F, t7255: F, t7445: F, t26024: F, t1860: F, t2110: F, t22549: F, t24514: F, t24517: F, t26009: F, t26016: F, t26028: F, t26070: F, t26073: F, t26076: F, t6486: F, t7256: F, t7259: F, t7428: F, t7978: F) -> (F, F, F) {
    let t27290 = t2165 * t4072;
    let t27293 = t8103 * t671;
    let t27298 = t2109 * t26012;
    let t27303 = t7974 * t6509;
    let t27308 = t7255 * t7445;
    let t27311 = t2109 * t26024;
    let t27326 = -F::new(5.0) * t24514 * t26009 - F::new(5.0) / F::new(3.0) * t22549 * t27298 - F::new(5.0) / F::new(3.0) * t26016 * t24517 - t1860 * t27303 / F::new(6.0) - t6486 * t7978 / F::new(6.0) - t1860 * t27308 / F::new(6.0) - t1860 * t27311 / F::new(6.0) - t26028 * t2110 / F::new(6.0) - t7428 * t7256 / F::new(6.0) - t7428 * t7259 / F::new(6.0) + t26070 * t2110 / F::new(3.0) + t26073 * t2110 / F::new(3.0) + t26076 * t2110 / F::new(3.0);
    (t27290, t27293, t27326)
}
