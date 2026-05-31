//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1106/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1106<F: Float>(t1052: F, t1920: F, t1956: F, t23327: F, t25755: F, t25778: F, t30798: F, t32909: F, t32913: F, t32917: F, t32924: F, t32965: F, t32970: F, t32973: F, t32976: F, t32981: F, t4557: F, t6687: F, t8407: F) -> F {
    let t32984 = -F::cast_from(6.0_f64) * t1052 * t32909 + F::cast_from(2.0_f64) * t1052 * t32913 + F::cast_from(4.0_f64) * t1052 * t32917 - F::cast_from(2.0_f64) * t25755 * t1956 + F::cast_from(0.16449340668482264365e-1_f64) * t1920 * t32924 - t1052 * t32965 - F::cast_from(2.0_f64) * t25778 * t1956 - F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t32970 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t32973 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t32976 - t4557 * t8407 + t30798 + F::cast_from(0.3289868133696452873e-1_f64) * t6687 * t32981;
    t32984
}
