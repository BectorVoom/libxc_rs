//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1736/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1736<F: Float>(t12850: F, t12854: F, t12860: F, t12861: F, t12889: F, t12891: F, t12894: F, t12895: F, t12899: F, t12903: F, t12906: F, t1877: F, t2522: F, t2553: F, t4310: F, t4314: F, t776: F, t868: F, t9457: F, t9462: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F) -> F {
    let t12907 = -F::new(2.0) * t12854 * t1877 * t868 + F::new(6.0) * t12895 * t2522 * t776 + F::new(12.0) * t12899 * t4314 * t776 + F::new(3.0) * t2522 * t2553 * t4310 + F::new(6.0) * t12903 * t4314 + t12850 - t12860 + t12861 + t12889 + t12891 + t12894 - t12906 - t9457 + t9462 - t9469 + t9476 + t9484 - t9496 - t9715;
    t12907
}
