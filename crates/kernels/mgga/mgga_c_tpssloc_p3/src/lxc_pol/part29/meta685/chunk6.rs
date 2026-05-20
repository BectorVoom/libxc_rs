//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2344/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2344<F: Float>(t22550: F, t7974: F, t2109: F, t90247: F, t1419: F, t2274: F, t12606: F, t12648: F, t12652: F, t14165: F, t1860: F, t1864: F, t2108: F, t2110: F, t2244: F, t2250: F, t22549: F, t24498: F, t24505: F, t24508: F, t26009: F, t26028: F, t27303: F, t27356: F, t27364: F, t27365: F, t608: F, t6486: F, t6509: F, t67: F, t7251: F, t7256: F, t7259: F, t7428: F, t83803: F, t85539: F, t90121: F, t9239: F) -> F {
    let t96135 = t7974 * t22550;
    let t96138 = t2109 * t90247;
    let t96157 = t1419 * t2274;
    let t96180 = -F::new(10.0) / F::new(3.0) * t22549 * t96135 - F::new(10.0) / F::new(3.0) * t22549 * t96138 + F::new(20.0) * t9239 * t608 * t2108 * t26009 - t90121 * t2110 / F::new(6.0) - t26028 * t7256 / F::new(3.0) - t26028 * t7259 / F::new(3.0) - t7428 * t24505 / F::new(6.0) - t7428 * t24508 / F::new(3.0) - t6486 * t27365 / F::new(3.0) - t1860 * (-F::new(20.0) / F::new(27.0) * t96157 * t2244 + F::new(20.0) / F::new(9.0) * t27356 * t2250 + F::new(5.0) / F::new(108.0) * t85539 * t14165 + F::new(5.0) / F::new(9.0) * t24498 * t12652 + F::new(5.0) / F::new(18.0) * t24498 * t12648 - F::new(5.0) / F::new(6.0) * t7251 * t12606 + t83803) * t67 * t1864 / F::new(6.0) - t1860 * t27364 * t6509 / F::new(3.0) - t6486 * t27303 / F::new(3.0);
    t96180
}
