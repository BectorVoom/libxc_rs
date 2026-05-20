//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2029/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2029<F: Float>(t91486: F, t12030: F, t16452: F, t1843: F, t2092: F, t24139: F, t26224: F, t26989: F, t27068: F, t3889: F, t5215: F, t55150: F, t7937: F, t81365: F, t81375: F, t84700: F, t91478: F, t91482: F) -> F {
    let t93873 = F::cast_from(0.3289868133696452873e-1_f64) * t91486;
    let t93879 = -F::new(12.0) * t26224 * t26989 * t16452 + F::new(2.0) * t27068 * t3889 + F::cast_from(0.3289868133696452873e-1_f64) * t81365 + F::cast_from(0.9869604401089358619e-1_f64) * t91478 - F::cast_from(0.3289868133696452873e-1_f64) * t91482 + t93873 - t55150 * t2092 - t12030 * t7937 - F::cast_from(0.51175726524167044691e0_f64) * t81375 - t84700 * t1843 - t5215 * t24139;
    t93879
}
