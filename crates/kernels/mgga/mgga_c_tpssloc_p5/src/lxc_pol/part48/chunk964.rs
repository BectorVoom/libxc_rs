//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 964/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk964<F: Float>(t23030: F, t31319: F, t23168: F, t31367: F, t112893: F, t112902: F, t112905: F, t114741: F, t114877: F, t114880: F, t114882: F, t114889: F, t1912: F, t2054: F, t218: F, t23215: F, t259: F, t2713: F, t31311: F, t7087: F, t82071: F, t85079: F, t85146: F, t85152: F, t8563: F, t9593: F) -> F {
    let t114891 = t23030 * t31319;
    let t114892 = F::cast_from(0.26044789391763585244e-1_f64) * t114891;
    let t114900 = t23168 * t31367;
    let t114902 = -t85079 * t1912 - F::cast_from(2.0_f64) * t9593 * t8563 - t82071 * t2054 - F::cast_from(0.6579736267392905746e-1_f64) * t114877 + F::cast_from(0.3289868133696452873e-1_f64) * t114880 + F::cast_from(0.38381794893125283518e-1_f64) * t114882 - F::cast_from(2.0_f64) * t85146 * t1912 - t112893 + F::cast_from(0.82246703342411321825e-2_f64) * t114889 + t114892 + t218 * t114741 * t259 - t85152 * t1912 + t112902 + F::cast_from(4.0_f64) * t2713 * t31311 - F::cast_from(6.0_f64) * t7087 * t23215 + t112905 + F::cast_from(0.76763589786250567036e-1_f64) * t114900;
    t114902
}
