//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2356/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2356<F: Float>(t48338: F, t10263: F, t4528: F, t12606: F, t2989: F, t10241: F, t13861: F, t1600: F, t2986: F, t2988: F, t3008: F, t3014: F, t343: F, t42554: F, t43061: F, t4514: F, t4540: F, t4543: F, t4546: F, t48329: F, t48336: F, t973: F) -> F {
    let t48339 = F::cast_from(0.14814814814814814814e-2_f64) * t48338;
    let t48342 = t10263 * t4528;
    let t48357 = t2989 * t12606;
    let t48361 = t48329 - F::cast_from(0.24999999999999999999e-2_f64) * t973 * t4546 * t4540 * t3008 * t343 - F::cast_from(0.3086419753086419753e-3_f64) * t48336 - t48339 + F::cast_from(0.38024691358024691358e-1_f64) * t42554 * t1600 - F::cast_from(0.81481481481481481478e-2_f64) * t48342 - F::cast_from(0.24444444444444444444e-1_f64) * t10263 * t4543 - F::cast_from(0.24999999999999999999e-2_f64) * t973 * t4546 * t4540 * t3014 * t343 - F::cast_from(0.27777777777777777777e-3_f64) * t2986 * t43061 * t4514 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t10241 * t13861 - F::cast_from(0.83333333333333333331e-3_f64) * t2986 * t2988 * t48357;
    t48361
}
