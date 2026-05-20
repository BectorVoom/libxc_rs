//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2606/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2606<F: Float>(t1227: F, t13969: F, t15649: F, t43763: F, t44827: F, t11539: F, t1174: F, t14740: F, t11694: F, t11739: F, t1215: F, t1653: F, t3493: F, t3577: F, t3578: F, t44936: F, t45119: F, t45128: F, t45197: F, t4582: F, t4728: F, t48497: F, t52183: F, t52704: F, t52893: F, t52897: F, t52903: F, t52906: F, t52908: F, t52911: F) -> F {
    let t52917 = t1227 * t13969 * t15649;
    let t52919 = t44827 * t43763;
    let t52926 = t1174 * t11539 * t14740;
    let t52928 = -t45119 * t3578 * t1653 * t11739 / F::new(4608.0) - F::new(5.0) / F::new(1728.0) * t52893 * t45128 * t52183 + F::new(3.0) / F::new(512.0) * t45197 * t52897 * t52704 * t3493 * t1215 - t52903 * t11694 / F::new(288.0) - t52906 / F::new(144.0) + t52908 / F::new(768.0) - t3577 * t3578 * t4728 * t52911 / F::new(768.0) - t52917 / F::new(576.0) + F::new(55.0) / F::new(15552.0) * t1227 * t4582 * t52919 * t48497 + t44936 / F::new(108.0) + t52926 / F::new(216.0);
    t52928
}
