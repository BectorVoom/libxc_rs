//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2271/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2271<F: Float>(t1433: F, t2307: F, t72: F, t26083: F, t9231: F, t2240: F, t26043: F, t33: F, t12606: F, t12648: F, t12652: F, t1409: F, t14165: F, t1860: F, t1864: F, t22489: F, t22490: F, t22502: F, t22505: F, t22513: F, t22516: F, t22537: F, t26044: F, t26045: F, t26048: F, t3961: F, t3966: F, t6486: F, t6490: F, t6492: F, t6500: F, t6509: F, t67: F, t7435: F, t7441: F, t7446: F, t83788: F, t83791: F, t83796: F, t83803: F) -> F {
    let t90297 = t72 * t1433 * t2307;
    let t90308 = t9231 * t26083;
    let t90312 = t2240 * t33 * t26043;
    let t90315 = -t6486 * t26045 / F::new(3.0) - t6486 * t26048 / F::new(3.0) - t1860 * (F::new(220.0) / F::new(27.0) * t83788 * t1409 - F::new(40.0) / F::new(27.0) * t83791 * t3961 - F::new(40.0) / F::new(9.0) * t22502 * t3966 - F::new(5.0) / F::new(108.0) * t83796 * t14165 + F::new(5.0) / F::new(9.0) * t22505 * t12652 + F::new(5.0) / F::new(18.0) * t22505 * t12648 + F::new(5.0) / F::new(6.0) * t6500 * t12606 + t83803) * t67 * t1864 / F::new(6.0) - t1860 * t26044 * t6509 / F::new(3.0) - t1860 * t7441 * t22489 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t6490 * t90297 + t22537 * t7446 / F::new(3.0) + t7435 * t22513 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t7435 * t22516 + t7435 * t22490 / F::new(3.0) + F::new(5.0) / F::new(3.0) * t90308 * t6492 + F::new(5.0) / F::new(3.0) * t90312 * t6492;
    t90315
}
